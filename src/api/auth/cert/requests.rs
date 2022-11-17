use rustify_derive::Endpoint;

/// ## Login with Cert
/// Issues a Vault token based on the presented credentials.
///
/// * Path: /auth/cert/login
/// * Method: POST
/// * Response: N/A
/// * Reference: https://developer.hashicorp.com/vault/api-docs/auth/cert#login-with-tls-certificate-method
#[derive(Builder, Debug, Endpoint)]
#[endpoint(path = "/auth/cert/login", method = "POST", builder = "true")]
#[builder(setter(into))]
pub struct LoginWithTSLCertificateRequest {
  pub name: String
}
