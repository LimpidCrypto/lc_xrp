use loco_rs::controller::ErrorDetail;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletException {
    #[error("Failed to create account")]
    FailedToCreateAccount,
    #[error("Failed to fund wallet: {0:?}")]
    FailedToFundWallet(#[from] reqwest::Error),
}

impl Into<ErrorDetail> for WalletException {
    fn into(self) -> ErrorDetail {
        ErrorDetail::with_reason(&self.to_string())
    }
}
