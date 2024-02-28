pub mod exceptions;
pub mod faucet;

use loco_rs::prelude::*;

pub fn routes() -> Routes {
    Routes::new()
        .prefix("wallet")
        .add("/faucet", get(faucet::generate_faucet_wallet))
}
