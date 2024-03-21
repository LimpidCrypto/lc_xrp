#![allow(clippy::unused_async)]
use axum::http::StatusCode;
use loco_rs::prelude::*;

use crate::{
    utils::networks::get_faucet_network_url, variants::network::Network,
    views::faucet::FaucetResponse,
};

use super::{create_wallet, exceptions::WalletException};

pub async fn generate_faucet_wallet(
    Path(network): Path<Network>,
    State(_ctx): State<AppContext>,
) -> Result<Json<FaucetResponse>> {
    let (wallet, x_address) = create_wallet()?;
    let network_faucet_url = match get_faucet_network_url(network) {
        Ok(url) => url,
        Err(error) => {
            return Err(Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                error.into(),
            ))
        }
    };
    let mut funded_response =
        fund_wallet(network_faucet_url, wallet.classic_address.clone()).await?;
    if funded_response.account.xaddress.is_none() {
        funded_response.account.xaddress = Some(x_address);
    }
    if funded_response.account.secret.is_none() {
        funded_response.account.secret = Some(wallet.seed.clone());
    }

    format::json(funded_response)
}

async fn fund_wallet(network_faucet_url: String, address: String) -> Result<FaucetResponse> {
    let client = reqwest::Client::new();
    let faucet_result = client
        .post(&network_faucet_url)
        .body(format!("{{\"destination\":\"{}\"}}", address))
        .send()
        .await;
    match faucet_result {
        Ok(response) => {
            if response.status().is_success() {
                let response_text = match response.text().await {
                    Ok(text) => text,
                    Err(error) => {
                        return Err(Error::CustomError(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            WalletException::FailedToFundWallet(error).into(),
                        ))
                    }
                };
                let response_json: FaucetResponse = match serde_json::from_str(&response_text) {
                    Ok(json) => json,
                    Err(error) => return Err(Error::JSON(error)),
                };
                Ok(response_json)
            } else {
                Err(Error::Message(format!(
                    "Failed to fund wallet: {}",
                    response.status()
                )))
            }
        }
        Err(error) => Err(Error::wrap(error)),
    }
}
