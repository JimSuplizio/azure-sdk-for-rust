use crate::{
    federated_credentials_flow, token_credentials::cache::TokenCache, TokenCredentialOptions,
};
use azure_core::{
    auth::{AccessToken, Secret, TokenCredential},
    error::{ErrorKind, ResultExt},
    HttpClient,
};
use std::{str, sync::Arc, time::Duration};
use time::OffsetDateTime;

/// Enables authentication to Azure Active Directory using a client secret that was generated for an App Registration.
///
/// More information on how to configure a client secret can be found here:
/// <https://docs.microsoft.com/azure/active-directory/develop/quickstart-configure-app-access-web-apis#add-credentials-to-your-web-application>

#[derive(Debug)]
pub struct WorkloadIdentityCredential {
    http_client: Arc<dyn HttpClient>,
    tenant_id: String,
    client_id: String,
    token: Secret,
    options: TokenCredentialOptions,
    cache: TokenCache,
}

impl WorkloadIdentityCredential {
    /// Create a new `WorkloadIdentityCredential`
    pub fn new<T>(
        http_client: Arc<dyn HttpClient>,
        tenant_id: String,
        client_id: String,
        token: T,
    ) -> Self
    where
        T: Into<Secret>,
    {
        Self {
            http_client,
            tenant_id,
            client_id,
            token: token.into(),
            options: TokenCredentialOptions::default(),
            cache: TokenCache::new(),
        }
    }

    /// set `TokenCredentialOptions`
    pub fn set_options(&mut self, options: TokenCredentialOptions) {
        self.options = options;
    }

    async fn get_token(&self, resource: &str) -> azure_core::Result<AccessToken> {
        let mut resource = resource.to_owned();
        if !resource.ends_with("/.default") {
            if resource.ends_with('/') {
                resource.push_str(".default");
            } else {
                resource.push_str("/.default");
            }
        }

        let res: AccessToken = federated_credentials_flow::perform(
            self.http_client.clone(),
            &self.client_id,
            self.token.secret(),
            &[&resource],
            &self.tenant_id,
            self.options.authority_host(),
        )
        .await
        .map(|r| {
            AccessToken::new(
                r.access_token().clone(),
                OffsetDateTime::now_utc() + Duration::from_secs(r.expires_in),
            )
        })
        .context(ErrorKind::Credential, "request token error")?;
        Ok(res)
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for WorkloadIdentityCredential {
    async fn get_token(&self, resource: &str) -> azure_core::Result<AccessToken> {
        self.cache
            .get_token(resource, self.get_token(resource))
            .await
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        self.cache.clear().await
    }
}
