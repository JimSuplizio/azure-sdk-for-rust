// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

// BEGIN GENERATED CODE -- do not edit from here till END
mod generated;

pub mod clients {
    pub use crate::generated::clients::{KeyClient, KeyClientOptions};
}

pub mod models {
    pub use crate::generated::clients::method_options::{
        KeyClientBackupKeyOptions, KeyClientCreateKeyOptions, KeyClientDecryptOptions,
        KeyClientDeleteKeyOptions, KeyClientEncryptOptions, KeyClientGetDeletedKeyOptions,
        KeyClientGetDeletedKeysOptions, KeyClientGetKeyAttestationOptions, KeyClientGetKeyOptions,
        KeyClientGetKeyRotationPolicyOptions, KeyClientGetKeyVersionsOptions,
        KeyClientGetKeysOptions, KeyClientGetRandomBytesOptions, KeyClientImportKeyOptions,
        KeyClientPurgeDeletedKeyOptions, KeyClientRecoverDeletedKeyOptions,
        KeyClientReleaseOptions, KeyClientRestoreKeyOptions, KeyClientRotateKeyOptions,
        KeyClientSignOptions, KeyClientUnwrapKeyOptions, KeyClientUpdateKeyOptions,
        KeyClientUpdateKeyRotationPolicyOptions, KeyClientVerifyOptions, KeyClientWrapKeyOptions,
    };
    pub use crate::generated::enums::*;
    pub use crate::generated::models::*;
}

pub use crate::generated::clients::{KeyClient, KeyClientOptions};
// END GENERATED CODE

mod resource;
pub use resource::*;
