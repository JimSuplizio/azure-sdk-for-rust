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
    pub fn job(&self) -> job::Client {
        job::Client(self.clone())
    }
}
pub mod job {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn get_statistics(&self, job_identity: impl Into<String>) -> get_statistics::Builder {
            get_statistics::Builder {
                client: self.0.clone(),
                job_identity: job_identity.into(),
            }
        }
        pub fn get_debug_data_path(&self, job_identity: impl Into<String>) -> get_debug_data_path::Builder {
            get_debug_data_path::Builder {
                client: self.0.clone(),
                job_identity: job_identity.into(),
            }
        }
        pub fn build(&self, parameters: impl Into<models::JobInformation>) -> build::Builder {
            build::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
            }
        }
        pub fn cancel(&self, job_identity: impl Into<String>) -> cancel::Builder {
            cancel::Builder {
                client: self.0.clone(),
                job_identity: job_identity.into(),
            }
        }
        pub fn get(&self, job_identity: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                job_identity: job_identity.into(),
            }
        }
        pub fn create(&self, job_identity: impl Into<String>, parameters: impl Into<models::JobInformation>) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                job_identity: job_identity.into(),
                parameters: parameters.into(),
            }
        }
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                filter: None,
                top: None,
                skip: None,
                expand: None,
                select: None,
                orderby: None,
                count: None,
                search: None,
                format: None,
            }
        }
    }
    pub mod get_statistics {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::JobStatistics;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_identity: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!("{}/Jobs/{}/GetStatistics", this.client.endpoint(), &this.job_identity);
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::GET);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "2016-03-20-preview");
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
                                let rsp_value: models::JobStatistics = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_debug_data_path {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::JobDataPath;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_identity: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!("{}/Jobs/{}/GetDebugDataPath", this.client.endpoint(), &this.job_identity);
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::POST);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "2016-03-20-preview");
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
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::JobDataPath = serde_json::from_slice(&rsp_body)?;
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
    pub mod build {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::JobInformation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::JobInformation,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!("{}/BuildJob", this.client.endpoint(),);
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::POST);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "2016-03-20-preview");
                        req_builder = req_builder.header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
                                let rsp_value: models::JobInformation = serde_json::from_slice(&rsp_body)?;
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
    pub mod cancel {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_identity: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!("{}/Jobs/{}/CancelJob", this.client.endpoint(), &this.job_identity);
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::POST);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "2016-03-20-preview");
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
                            http::StatusCode::OK => Ok(()),
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
    pub mod get {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::JobInformation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_identity: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!("{}/Jobs/{}", this.client.endpoint(), &this.job_identity);
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::GET);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "2016-03-20-preview");
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
                                let rsp_value: models::JobInformation = serde_json::from_slice(&rsp_body)?;
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
    pub mod create {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::JobInformation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_identity: String,
            pub(crate) parameters: models::JobInformation,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!("{}/Jobs/{}", this.client.endpoint(), &this.job_identity);
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::PUT);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "2016-03-20-preview");
                        req_builder = req_builder.header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
                                let rsp_value: models::JobInformation = serde_json::from_slice(&rsp_body)?;
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
    pub mod list {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::JobInfoListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) skip: Option<i32>,
            pub(crate) expand: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) orderby: Option<String>,
            pub(crate) count: Option<bool>,
            pub(crate) search: Option<String>,
            pub(crate) format: Option<String>,
        }
        impl Builder {
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            pub fn count(mut self, count: bool) -> Self {
                self.count = Some(count);
                self
            }
            pub fn search(mut self, search: impl Into<String>) -> Self {
                self.search = Some(search.into());
                self
            }
            pub fn format(mut self, format: impl Into<String>) -> Self {
                self.format = Some(format.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<azure_core::prelude::Continuation>| {
                    let this = self.clone();
                    async move {
                        let url_str = &format!("{}/Jobs", this.client.endpoint(),);
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
                                    url.query_pairs_mut().append_pair("api-version", "2016-03-20-preview");
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
                                url.query_pairs_mut().append_pair("api-version", "2016-03-20-preview");
                                if let Some(filter) = &this.filter {
                                    url.query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    url.query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(skip) = &this.skip {
                                    url.query_pairs_mut().append_pair("$skip", &skip.to_string());
                                }
                                if let Some(expand) = &this.expand {
                                    url.query_pairs_mut().append_pair("$expand", expand);
                                }
                                if let Some(select) = &this.select {
                                    url.query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(orderby) = &this.orderby {
                                    url.query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(count) = &this.count {
                                    url.query_pairs_mut().append_pair("$count", &count.to_string());
                                }
                                if let Some(search) = &this.search {
                                    url.query_pairs_mut().append_pair("$search", search);
                                }
                                if let Some(format) = &this.format {
                                    url.query_pairs_mut().append_pair("$format", format);
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
                                let rsp_value: models::JobInfoListResult = serde_json::from_slice(&rsp_body)?;
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
