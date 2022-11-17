use async_trait::async_trait;
use vaultrs::{api::AuthInfo, client::Client, error::ClientError};

use crate::LoginMethod;

/// A login method which uses AppRole credentials for obtaining a new token.
#[derive(Debug)]
pub struct ClientCert {
}

impl ClientCert {
    pub fn new() -> Self {
        ClientCert {}
    }
}

#[async_trait]
impl LoginMethod for ClientCert {
    async fn login(&self, client: &impl Client, name: &str) -> Result<AuthInfo, ClientError> {
        vaultrs::auth::cert::login(
            client,
            name
        )
        .await
    }
}
