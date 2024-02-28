use std::path::Path;

use async_trait::async_trait;
use axum::Extension;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    boot::{create_app, BootResult, StartMode},
    config::Database,
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    worker::Processor,
    Result,
};
use migration::Migrator;
use sea_orm::DatabaseConnection;

use crate::{controllers, initializers, models::_entities::notes, tasks};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![Box::new(
            initializers::view_engine::ViewEngineInitializer,
        )])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .prefix("/api/v1")
            .add_route(controllers::notes::routes())
    }

    fn connect_workers<'a>(_p: &'a mut Processor, _ctx: &'a AppContext) {}

    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::seed::SeedData);
    }

    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        truncate_table(db, notes::Entity).await?;
        Ok(())
    }

    async fn seed(db: &DatabaseConnection, base: &Path) -> Result<()> {
        db::seed::<notes::ActiveModel>(db, &base.join("notes.yaml").display().to_string()).await?;
        Ok(())
    }

    async fn after_routes(router: axum::Router, _ctx: &AppContext) -> Result<axum::Router> {
        let user_db_uri: Option<&str> = option_env!("USER_DATABASE_URL");
        if let Some(uri) = user_db_uri {
            let user_db_config = Database {
                uri: uri.to_string(),
                enable_logging: false,
                min_connections: 2,
                max_connections: 10,
                connect_timeout: 500,
                idle_timeout: 500,
                auto_migrate: false,
                dangerously_truncate: false,
                dangerously_recreate: false,
            };
            let user_db = db::connect(&user_db_config).await?;

            Ok(router.layer(Extension(user_db)))
        } else {
            Ok(router)
        }
    }
}
