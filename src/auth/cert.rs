use crate::{
    api::{
        self,
        auth::cert::requests::{LoginWithTSLCertificateRequest},
        AuthInfo,
    },
    client::Client,
    error::ClientError,
};

// Login with client certificates specified in Client
//
// See [LoginWithTSLCertificateRequest]
#[instrument(skip(client), err)]
pub async fn login(
    client: &impl Client,
    name: &str
) -> Result<AuthInfo, ClientError> {
    let endpoint = LoginWithTSLCertificateRequest::builder()
        .name(name)
        .build()
        .unwrap();
    api::auth(client, endpoint).await
}
