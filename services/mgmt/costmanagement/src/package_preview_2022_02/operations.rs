#![doc = "generated by AutoRust"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
use super::models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: impl Into<azure_core::Request>) -> azure_core::error::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn operations(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn price_sheet(&self) -> price_sheet::Client {
        price_sheet::Client(self.clone())
    }
}
pub mod price_sheet {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn download(
            &self,
            billing_account_name: impl Into<String>,
            billing_profile_name: impl Into<String>,
            invoice_name: impl Into<String>,
        ) -> download::Builder {
            download::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                invoice_name: invoice_name.into(),
            }
        }
        pub fn download_by_billing_profile(
            &self,
            billing_account_name: impl Into<String>,
            billing_profile_name: impl Into<String>,
        ) -> download_by_billing_profile::Builder {
            download_by_billing_profile::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
            }
        }
    }
    pub mod download {
        use super::models;
        use azure_core::error::ResultExt;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            Ok200(models::DownloadUrl),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) invoice_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = & format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/invoices/{}/providers/Microsoft.CostManagement/pricesheets/default/download" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name , & this . invoice_name) ;
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::POST);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "2022-02-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder
                            .body(req_body)
                            .context(azure_core::error::ErrorKind::Other, "build request")?;
                        let rsp = this
                            .client
                            .send(req)
                            .await
                            .context(azure_core::error::ErrorKind::Io, "execute request")?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::ACCEPTED => Ok(Response::Accepted202),
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::DownloadUrl = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code.as_u16(),
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod download_by_billing_profile {
        use super::models;
        use azure_core::error::ResultExt;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            Ok200(models::DownloadUrl),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = & format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/providers/Microsoft.CostManagement/pricesheets/default/download" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name) ;
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::POST);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "2022-02-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder
                            .body(req_body)
                            .context(azure_core::error::ErrorKind::Other, "build request")?;
                        let rsp = this
                            .client
                            .send(req)
                            .await
                            .context(azure_core::error::ErrorKind::Io, "execute request")?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::ACCEPTED => Ok(Response::Accepted202),
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::DownloadUrl = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code.as_u16(),
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::OperationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<azure_core::prelude::Continuation>| {
                    let this = self.clone();
                    async move {
                        let url_str = &format!("{}/providers/Microsoft.CostManagement/operations", this.client.endpoint(),);
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::Other, "build request")?;
                        let mut req_builder = http::request::Builder::new();
                        let rsp = match continuation {
                            Some(token) => {
                                url.set_path("");
                                url = url
                                    .join(&token.into_raw())
                                    .context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                                let has_api_version_already = url.query_pairs().any(|(k, _)| k == "api-version");
                                if !has_api_version_already {
                                    url.query_pairs_mut().append_pair("api-version", "2022-02-01-preview");
                                }
                                req_builder = req_builder.uri(url.as_str());
                                req_builder = req_builder.method(http::Method::GET);
                                let credential = this.client.token_credential();
                                let token_response = credential
                                    .get_token(&this.client.scopes().join(" "))
                                    .await
                                    .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                                req_builder =
                                    req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                                let req_body = azure_core::EMPTY_BODY;
                                let req = req_builder
                                    .body(req_body)
                                    .context(azure_core::error::ErrorKind::Other, "build request")?;
                                this.client
                                    .send(req)
                                    .await
                                    .context(azure_core::error::ErrorKind::Io, "execute request")?
                            }
                            None => {
                                req_builder = req_builder.method(http::Method::GET);
                                let credential = this.client.token_credential();
                                let token_response = credential
                                    .get_token(&this.client.scopes().join(" "))
                                    .await
                                    .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                                req_builder =
                                    req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                                url.query_pairs_mut().append_pair("api-version", "2022-02-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req_builder = req_builder.uri(url.as_str());
                                let req = req_builder
                                    .body(req_body)
                                    .context(azure_core::error::ErrorKind::Other, "build request")?;
                                this.client
                                    .send(req)
                                    .await
                                    .context(azure_core::error::ErrorKind::Io, "execute request")?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::OperationListResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code.as_u16(),
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
