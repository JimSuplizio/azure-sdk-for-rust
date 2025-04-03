// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.
// Code generated by Microsoft (R) Rust Code Generator. DO NOT EDIT.

use crate::generated::{
    clients::BlobClient,
    models::{
        BlobContainerClientAcquireLeaseOptions, BlobContainerClientAcquireLeaseResult,
        BlobContainerClientBreakLeaseOptions, BlobContainerClientBreakLeaseResult,
        BlobContainerClientChangeLeaseOptions, BlobContainerClientChangeLeaseResult,
        BlobContainerClientCreateOptions, BlobContainerClientDeleteOptions,
        BlobContainerClientFilterBlobsOptions, BlobContainerClientGetAccessPolicyOptions,
        BlobContainerClientGetAccountInfoOptions, BlobContainerClientGetAccountInfoResult,
        BlobContainerClientGetPropertiesOptions, BlobContainerClientGetPropertiesResult,
        BlobContainerClientListBlobFlatSegmentOptions,
        BlobContainerClientListBlobHierarchySegmentOptions, BlobContainerClientReleaseLeaseOptions,
        BlobContainerClientReleaseLeaseResult, BlobContainerClientRenameOptions,
        BlobContainerClientRenameResult, BlobContainerClientRenewLeaseOptions,
        BlobContainerClientRenewLeaseResult, BlobContainerClientRestoreOptions,
        BlobContainerClientRestoreResult, BlobContainerClientSetAccessPolicyOptions,
        BlobContainerClientSetAccessPolicyResult, BlobContainerClientSetMetadataOptions,
        BlobContainerClientSetMetadataResult, FilterBlobSegment, ListBlobsFlatSegmentResponse,
        ListBlobsHierarchySegmentResponse, SignedIdentifier,
    },
};
use azure_core::{
    credentials::TokenCredential,
    date,
    fmt::SafeDebug,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        ClientOptions, Context, Method, Pipeline, Request, RequestContent, Response, Url,
    },
    Result,
};
use std::sync::Arc;

pub struct BlobContainerClient {
    pub(crate) container_name: String,
    pub(crate) endpoint: Url,
    pub(crate) pipeline: Pipeline,
    pub(crate) version: String,
}

/// Options used when creating a `BlobContainerClient`
#[derive(Clone, SafeDebug)]
pub struct BlobContainerClientOptions {
    /// Allows customization of the client.
    pub client_options: ClientOptions,
    /// Specifies the version of the operation to use for this request.
    pub version: String,
}

impl BlobContainerClient {
    /// Creates a new BlobContainerClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - Service host
    /// * `credential` - An implementation of [`TokenCredential`](azure_core::credentials::TokenCredential) that can provide an
    ///   Entra ID token to use when authenticating.
    /// * `container_name` - The name of the container.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        credential: Arc<dyn TokenCredential>,
        container_name: String,
        options: Option<BlobContainerClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();
        let mut endpoint = Url::parse(endpoint)?;
        if !endpoint.scheme().starts_with("http") {
            return Err(azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                format!("{endpoint} must use http(s)"),
            ));
        }
        endpoint.set_query(None);
        let auth_policy: Arc<dyn Policy> = Arc::new(BearerTokenCredentialPolicy::new(
            credential,
            vec!["https://storage.azure.com/.default"],
        ));
        Ok(Self {
            container_name,
            endpoint,
            version: options.version,
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                options.client_options,
                Vec::default(),
                vec![auth_policy],
            ),
        })
    }

    /// Returns the Url associated with this client.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// The Acquire Lease operation requests a new lease on a container. The lease lock duration can be 15 to 60 seconds, or can
    /// be infinite.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    pub async fn acquire_lease(
        &self,
        options: Option<BlobContainerClientAcquireLeaseOptions<'_>>,
    ) -> Result<Response<BlobContainerClientAcquireLeaseResult>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut()
            .append_key_only("acquire")
            .append_pair("comp", "lease")
            .append_pair("restype", "container");
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/xml");
        if let Some(if_modified_since) = options.if_modified_since {
            request.insert_header("if-modified-since", date::to_rfc7231(&if_modified_since));
        }
        if let Some(if_unmodified_since) = options.if_unmodified_since {
            request.insert_header(
                "if-unmodified-since",
                date::to_rfc7231(&if_unmodified_since),
            );
        }
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        if let Some(duration) = options.duration {
            request.insert_header("x-ms-lease-duration", duration.to_string());
        }
        if let Some(proposed_lease_id) = options.proposed_lease_id {
            request.insert_header("x-ms-proposed-lease-id", proposed_lease_id);
        }
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// The Break Lease operation ends a lease and ensures that another client can't acquire a new lease until the current lease
    /// period has expired.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    pub async fn break_lease(
        &self,
        options: Option<BlobContainerClientBreakLeaseOptions<'_>>,
    ) -> Result<Response<BlobContainerClientBreakLeaseResult>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut()
            .append_key_only("break")
            .append_pair("comp", "lease")
            .append_pair("restype", "container");
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/xml");
        if let Some(if_modified_since) = options.if_modified_since {
            request.insert_header("if-modified-since", date::to_rfc7231(&if_modified_since));
        }
        if let Some(if_unmodified_since) = options.if_unmodified_since {
            request.insert_header(
                "if-unmodified-since",
                date::to_rfc7231(&if_unmodified_since),
            );
        }
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        if let Some(break_period) = options.break_period {
            request.insert_header("x-ms-lease-break-period", break_period.to_string());
        }
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// The Change Lease operation is used to change the ID of an existing lease.
    ///
    /// # Arguments
    ///
    /// * `lease_id` - Required. A lease ID for the source path. If specified, the source path must have an active lease and the
    ///   lease ID must match.
    /// * `proposed_lease_id` - Required. The proposed lease ID for the container.
    /// * `options` - Optional parameters for the request.
    pub async fn change_lease(
        &self,
        lease_id: &str,
        proposed_lease_id: &str,
        options: Option<BlobContainerClientChangeLeaseOptions<'_>>,
    ) -> Result<Response<BlobContainerClientChangeLeaseResult>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut()
            .append_key_only("change")
            .append_pair("comp", "lease")
            .append_pair("restype", "container");
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/xml");
        if let Some(if_modified_since) = options.if_modified_since {
            request.insert_header("if-modified-since", date::to_rfc7231(&if_modified_since));
        }
        if let Some(if_unmodified_since) = options.if_unmodified_since {
            request.insert_header(
                "if-unmodified-since",
                date::to_rfc7231(&if_unmodified_since),
            );
        }
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        request.insert_header("x-ms-lease-id", lease_id.to_owned());
        request.insert_header("x-ms-proposed-lease-id", proposed_lease_id.to_owned());
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Creates a new container under the specified account. If the container with the same name already exists, the operation
    /// fails.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    pub async fn create(
        &self,
        options: Option<BlobContainerClientCreateOptions<'_>>,
    ) -> Result<Response<()>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut().append_pair("restype", "container");
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/xml");
        if let Some(access) = options.access {
            request.insert_header("x-ms-blob-public-access", access.to_string());
        }
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        if let Some(default_encryption_scope) = options.default_encryption_scope {
            request.insert_header("x-ms-default-encryption-scope", default_encryption_scope);
        }
        if let Some(prevent_encryption_scope_override) = options.prevent_encryption_scope_override {
            request.insert_header(
                "x-ms-deny-encryption-scope-override",
                prevent_encryption_scope_override.to_string(),
            );
        }
        if let Some(metadata) = options.metadata {
            for (k, v) in &metadata {
                request.insert_header(format!("x-ms-meta-{}", k), v);
            }
        }
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// operation marks the specified container for deletion. The container and any blobs contained within it are later deleted
    /// during garbage collection
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    pub async fn delete(
        &self,
        options: Option<BlobContainerClientDeleteOptions<'_>>,
    ) -> Result<Response<()>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut().append_pair("restype", "container");
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Delete);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/xml");
        if let Some(if_modified_since) = options.if_modified_since {
            request.insert_header("if-modified-since", date::to_rfc7231(&if_modified_since));
        }
        if let Some(if_unmodified_since) = options.if_unmodified_since {
            request.insert_header(
                "if-unmodified-since",
                date::to_rfc7231(&if_unmodified_since),
            );
        }
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        if let Some(lease_id) = options.lease_id {
            request.insert_header("x-ms-lease-id", lease_id);
        }
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// The Filter Blobs operation enables callers to list blobs in a container whose tags match a given search expression. Filter
    /// blobs searches within the given container.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    pub async fn filter_blobs(
        &self,
        options: Option<BlobContainerClientFilterBlobsOptions<'_>>,
    ) -> Result<Response<FilterBlobSegment>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut()
            .append_pair("comp", "blobs")
            .append_pair("restype", "container");
        if let Some(include) = options.include {
            url.query_pairs_mut().append_pair(
                "include",
                &include
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(marker) = options.marker {
            url.query_pairs_mut().append_pair("marker", &marker);
        }
        if let Some(maxresults) = options.maxresults {
            url.query_pairs_mut()
                .append_pair("maxresults", &maxresults.to_string());
        }
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        if let Some(where_param) = options.where_param {
            url.query_pairs_mut().append_pair("where", &where_param);
        }
        let mut request = Request::new(url, Method::Get);
        request.insert_header("accept", "application/xml");
        request.insert_header("content-type", "application/xml");
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// gets the permissions for the specified container. The permissions indicate whether container data may be accessed publicly.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    pub async fn get_access_policy(
        &self,
        options: Option<BlobContainerClientGetAccessPolicyOptions<'_>>,
    ) -> Result<Response<Vec<SignedIdentifier>>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut()
            .append_pair("comp", "acl")
            .append_pair("restype", "container");
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Get);
        request.insert_header("accept", "application/xml");
        request.insert_header("content-type", "application/xml");
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        if let Some(lease_id) = options.lease_id {
            request.insert_header("x-ms-lease-id", lease_id);
        }
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Returns the sku name and account kind
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    pub async fn get_account_info(
        &self,
        options: Option<BlobContainerClientGetAccountInfoOptions<'_>>,
    ) -> Result<Response<BlobContainerClientGetAccountInfoResult>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut()
            .append_pair("comp", "properties")
            .append_pair("restype", "account");
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Get);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/xml");
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Returns a new instance of BlobClient.
    ///
    /// # Arguments
    ///
    /// * `blob_name` - The name of the blob.
    pub fn get_blob_client(&self, blob_name: String) -> BlobClient {
        BlobClient {
            blob_name,
            container_name: self.container_name.clone(),
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
        }
    }

    /// returns all user-defined metadata and system properties for the specified container. The data returned does not include
    /// the container's list of blobs
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    pub async fn get_properties(
        &self,
        options: Option<BlobContainerClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<BlobContainerClientGetPropertiesResult>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut().append_pair("restype", "container");
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Get);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/xml");
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        if let Some(lease_id) = options.lease_id {
            request.insert_header("x-ms-lease-id", lease_id);
        }
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// The List Blobs operation returns a list of the blobs under the specified container.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    pub async fn list_blob_flat_segment(
        &self,
        options: Option<BlobContainerClientListBlobFlatSegmentOptions<'_>>,
    ) -> Result<Response<ListBlobsFlatSegmentResponse>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut()
            .append_pair("comp", "list")
            .append_key_only("flat")
            .append_pair("restype", "container");
        if let Some(include) = options.include {
            url.query_pairs_mut().append_pair(
                "include",
                &include
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(marker) = options.marker {
            url.query_pairs_mut().append_pair("marker", &marker);
        }
        if let Some(maxresults) = options.maxresults {
            url.query_pairs_mut()
                .append_pair("maxresults", &maxresults.to_string());
        }
        if let Some(prefix) = options.prefix {
            url.query_pairs_mut().append_pair("prefix", &prefix);
        }
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Get);
        request.insert_header("accept", "application/xml");
        request.insert_header("content-type", "application/xml");
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// The List Blobs operation returns a list of the blobs under the specified container. A delimiter can be used to traverse
    /// a virtual hierarchy of blobs as though it were a file system.
    ///
    /// # Arguments
    ///
    /// * `delimiter` - When the request includes this parameter, the operation returns a BlobPrefix element in the response body
    ///   that acts as a placeholder for all blobs whose names begin with the same substring up to the appearance of the delimiter
    ///   character. The delimiter may be a single character or a string.
    /// * `options` - Optional parameters for the request.
    pub async fn list_blob_hierarchy_segment(
        &self,
        delimiter: &str,
        options: Option<BlobContainerClientListBlobHierarchySegmentOptions<'_>>,
    ) -> Result<Response<ListBlobsHierarchySegmentResponse>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut()
            .append_pair("comp", "list")
            .append_key_only("hierarchy")
            .append_pair("restype", "container");
        url.query_pairs_mut().append_pair("delimiter", delimiter);
        if let Some(include) = options.include {
            url.query_pairs_mut().append_pair(
                "include",
                &include
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(marker) = options.marker {
            url.query_pairs_mut().append_pair("marker", &marker);
        }
        if let Some(maxresults) = options.maxresults {
            url.query_pairs_mut()
                .append_pair("maxresults", &maxresults.to_string());
        }
        if let Some(prefix) = options.prefix {
            url.query_pairs_mut().append_pair("prefix", &prefix);
        }
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Get);
        request.insert_header("accept", "application/xml");
        request.insert_header("content-type", "application/xml");
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// The Release Lease operation frees the lease if it's no longer needed, so that another client can immediately acquire a
    /// lease against the container.
    ///
    /// # Arguments
    ///
    /// * `lease_id` - Required. A lease ID for the source path. If specified, the source path must have an active lease and the
    ///   lease ID must match.
    /// * `options` - Optional parameters for the request.
    pub async fn release_lease(
        &self,
        lease_id: &str,
        options: Option<BlobContainerClientReleaseLeaseOptions<'_>>,
    ) -> Result<Response<BlobContainerClientReleaseLeaseResult>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut()
            .append_pair("comp", "lease")
            .append_key_only("release")
            .append_pair("restype", "container");
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/xml");
        if let Some(if_modified_since) = options.if_modified_since {
            request.insert_header("if-modified-since", date::to_rfc7231(&if_modified_since));
        }
        if let Some(if_unmodified_since) = options.if_unmodified_since {
            request.insert_header(
                "if-unmodified-since",
                date::to_rfc7231(&if_unmodified_since),
            );
        }
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        request.insert_header("x-ms-lease-id", lease_id.to_owned());
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Renames an existing container.
    ///
    /// # Arguments
    ///
    /// * `source_container_name` - Required. Specifies the name of the container to rename.
    /// * `options` - Optional parameters for the request.
    pub async fn rename(
        &self,
        source_container_name: &str,
        options: Option<BlobContainerClientRenameOptions<'_>>,
    ) -> Result<Response<BlobContainerClientRenameResult>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut()
            .append_pair("comp", "rename")
            .append_pair("restype", "container");
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/xml");
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        request.insert_header(
            "x-ms-source-container-name",
            source_container_name.to_owned(),
        );
        if let Some(source_lease_id) = options.source_lease_id {
            request.insert_header("x-ms-source-lease-id", source_lease_id);
        }
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// The Renew Lease operation renews an existing lease.
    ///
    /// # Arguments
    ///
    /// * `lease_id` - Required. A lease ID for the source path. If specified, the source path must have an active lease and the
    ///   lease ID must match.
    /// * `options` - Optional parameters for the request.
    pub async fn renew_lease(
        &self,
        lease_id: &str,
        options: Option<BlobContainerClientRenewLeaseOptions<'_>>,
    ) -> Result<Response<BlobContainerClientRenewLeaseResult>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut()
            .append_pair("comp", "lease")
            .append_key_only("renew")
            .append_pair("restype", "container");
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/xml");
        if let Some(if_modified_since) = options.if_modified_since {
            request.insert_header("if-modified-since", date::to_rfc7231(&if_modified_since));
        }
        if let Some(if_unmodified_since) = options.if_unmodified_since {
            request.insert_header(
                "if-unmodified-since",
                date::to_rfc7231(&if_unmodified_since),
            );
        }
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        request.insert_header("x-ms-lease-id", lease_id.to_owned());
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Restores a previously-deleted container.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    pub async fn restore(
        &self,
        options: Option<BlobContainerClientRestoreOptions<'_>>,
    ) -> Result<Response<BlobContainerClientRestoreResult>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut()
            .append_pair("comp", "undelete")
            .append_pair("restype", "container");
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/xml");
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        if let Some(deleted_container_name) = options.deleted_container_name {
            request.insert_header("x-ms-deleted-container-name", deleted_container_name);
        }
        if let Some(deleted_container_version) = options.deleted_container_version {
            request.insert_header("x-ms-deleted-container-version", deleted_container_version);
        }
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// sets the permissions for the specified container. The permissions indicate whether blobs in a container may be accessed
    /// publicly.
    ///
    /// # Arguments
    ///
    /// * `container_acl` - The access control list for the container.
    /// * `options` - Optional parameters for the request.
    pub async fn set_access_policy(
        &self,
        container_acl: RequestContent<Vec<SignedIdentifier>>,
        options: Option<BlobContainerClientSetAccessPolicyOptions<'_>>,
    ) -> Result<Response<BlobContainerClientSetAccessPolicyResult>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut()
            .append_pair("comp", "acl")
            .append_pair("restype", "container");
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/xml");
        if let Some(if_modified_since) = options.if_modified_since {
            request.insert_header("if-modified-since", date::to_rfc7231(&if_modified_since));
        }
        if let Some(if_unmodified_since) = options.if_unmodified_since {
            request.insert_header(
                "if-unmodified-since",
                date::to_rfc7231(&if_unmodified_since),
            );
        }
        if let Some(access) = options.access {
            request.insert_header("x-ms-blob-public-access", access.to_string());
        }
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        if let Some(lease_id) = options.lease_id {
            request.insert_header("x-ms-lease-id", lease_id);
        }
        request.insert_header("x-ms-version", &self.version);
        request.set_body(container_acl);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// operation sets one or more user-defined name-value pairs for the specified container.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    pub async fn set_metadata(
        &self,
        options: Option<BlobContainerClientSetMetadataOptions<'_>>,
    ) -> Result<Response<BlobContainerClientSetMetadataResult>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join(&self.container_name)?;
        url.query_pairs_mut()
            .append_pair("comp", "metadata")
            .append_pair("restype", "container");
        if let Some(timeout) = options.timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/xml");
        if let Some(if_modified_since) = options.if_modified_since {
            request.insert_header("if-modified-since", date::to_rfc7231(&if_modified_since));
        }
        if let Some(client_request_id) = options.client_request_id {
            request.insert_header("x-ms-client-request-id", client_request_id);
        }
        if let Some(lease_id) = options.lease_id {
            request.insert_header("x-ms-lease-id", lease_id);
        }
        if let Some(metadata) = options.metadata {
            for (k, v) in &metadata {
                request.insert_header(format!("x-ms-meta-{}", k), v);
            }
        }
        request.insert_header("x-ms-version", &self.version);
        self.pipeline.send(&ctx, &mut request).await
    }
}

impl Default for BlobContainerClientOptions {
    fn default() -> Self {
        Self {
            client_options: ClientOptions::default(),
            version: String::from("2025-01-05"),
        }
    }
}
