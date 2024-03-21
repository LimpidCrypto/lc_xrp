pub mod exceptions;
pub mod faucet;

use axum::http::StatusCode;
use loco_rs::prelude::*;
use xrpl::wallet::Wallet;

use self::exceptions::WalletException;

pub fn routes() -> Routes {
    Routes::new()
        .prefix("wallet")
        .add("/faucet", get(faucet::generate_faucet_wallet))
}

fn create_wallet() -> Result<(Wallet, String)> {
    let wallet = Wallet::create(None).map_err(|_| {
        Error::CustomError(
            StatusCode::INTERNAL_SERVER_ERROR,
            WalletException::FailedToCreateAccount.into(),
        )
    })?;
    let x_address = wallet.get_xaddress(None, true).map_err(|_| {
        Error::CustomError(
            StatusCode::INTERNAL_SERVER_ERROR,
            WalletException::FailedToCreateAccount.into(),
        )
    })?;

    Ok((wallet, x_address))
}
