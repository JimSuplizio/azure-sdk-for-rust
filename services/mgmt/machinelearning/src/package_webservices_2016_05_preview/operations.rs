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
    pub fn web_services(&self) -> web_services::Client {
        web_services::Client(self.clone())
    }
}
pub mod web_services {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            web_service_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                web_service_name: web_service_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        pub fn create_or_update(
            &self,
            resource_group_name: impl Into<String>,
            web_service_name: impl Into<String>,
            create_or_update_payload: impl Into<models::WebService>,
            subscription_id: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                web_service_name: web_service_name.into(),
                create_or_update_payload: create_or_update_payload.into(),
                subscription_id: subscription_id.into(),
            }
        }
        pub fn patch(
            &self,
            resource_group_name: impl Into<String>,
            web_service_name: impl Into<String>,
            patch_payload: impl Into<models::WebService>,
            subscription_id: impl Into<String>,
        ) -> patch::Builder {
            patch::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                web_service_name: web_service_name.into(),
                patch_payload: patch_payload.into(),
                subscription_id: subscription_id.into(),
            }
        }
        pub fn remove(
            &self,
            resource_group_name: impl Into<String>,
            web_service_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> remove::Builder {
            remove::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                web_service_name: web_service_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        pub fn list_keys(
            &self,
            resource_group_name: impl Into<String>,
            web_service_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_keys::Builder {
            list_keys::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                web_service_name: web_service_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        pub fn list_by_resource_group(
            &self,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_resource_group::Builder {
            list_by_resource_group::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                skiptoken: None,
            }
        }
        pub fn list(&self, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                skiptoken: None,
            }
        }
    }
    pub mod get {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::WebService;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) web_service_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearning/webServices/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.web_service_name
                        );
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::GET);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "2016-05-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
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
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::WebService = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
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
    pub mod create_or_update {
        use super::models;
        use azure_core::error::ResultExt;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::WebService),
            Created201(models::WebService),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) web_service_name: String,
            pub(crate) create_or_update_payload: models::WebService,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearning/webServices/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.web_service_name
                        );
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::PUT);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "2016-05-01-preview");
                        req_builder = req_builder.header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.create_or_update_payload)?;
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
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::WebService = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            http::StatusCode::CREATED => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::WebService = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
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
    pub mod patch {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::WebService;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) web_service_name: String,
            pub(crate) patch_payload: models::WebService,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearning/webServices/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.web_service_name
                        );
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::PATCH);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "2016-05-01-preview");
                        req_builder = req_builder.header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.patch_payload)?;
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
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::WebService = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
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
    pub mod remove {
        use super::models;
        use azure_core::error::ResultExt;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) web_service_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearning/webServices/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.web_service_name
                        );
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::DELETE);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "2016-05-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
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
                            http::StatusCode::NO_CONTENT => Ok(Response::NoContent204),
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
    pub mod list_keys {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::WebServiceKeys;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) web_service_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearning/webServices/{}/listKeys",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.web_service_name
                        );
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::GET);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "2016-05-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
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
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::WebServiceKeys = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
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
    pub mod list_by_resource_group {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::PaginatedWebServicesList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<azure_core::prelude::Continuation>| {
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearning/webServices",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name
                        );
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
                                    url.query_pairs_mut().append_pair("api-version", "2016-05-01-preview");
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
                                url.query_pairs_mut().append_pair("api-version", "2016-05-01-preview");
                                if let Some(skiptoken) = &this.skiptoken {
                                    url.query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
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
                                let rsp_value: models::PaginatedWebServicesList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::PaginatedWebServicesList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<azure_core::prelude::Continuation>| {
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/providers/Microsoft.MachineLearning/webServices",
                            this.client.endpoint(),
                            &this.subscription_id
                        );
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
                                    url.query_pairs_mut().append_pair("api-version", "2016-05-01-preview");
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
                                url.query_pairs_mut().append_pair("api-version", "2016-05-01-preview");
                                if let Some(skiptoken) = &this.skiptoken {
                                    url.query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
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
                                let rsp_value: models::PaginatedWebServicesList = serde_json::from_slice(&rsp_body)?;
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
