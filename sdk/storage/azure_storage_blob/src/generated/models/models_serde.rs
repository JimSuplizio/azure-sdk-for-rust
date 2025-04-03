// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.
// Code generated by Microsoft (R) Rust Code Generator. DO NOT EDIT.

use super::{BlobTags, BlockLookupList, QueryRequest};
use azure_core::{http::RequestContent, xml::to_xml, Result};

impl TryFrom<BlobTags> for RequestContent<BlobTags> {
    type Error = azure_core::Error;
    fn try_from(value: BlobTags) -> Result<Self> {
        RequestContent::try_from(to_xml(&value)?)
    }
}

impl TryFrom<BlockLookupList> for RequestContent<BlockLookupList> {
    type Error = azure_core::Error;
    fn try_from(value: BlockLookupList) -> Result<Self> {
        RequestContent::try_from(to_xml(&value)?)
    }
}

impl TryFrom<QueryRequest> for RequestContent<QueryRequest> {
    type Error = azure_core::Error;
    fn try_from(value: QueryRequest) -> Result<Self> {
        RequestContent::try_from(to_xml(&value)?)
    }
}

pub mod vec_encoded_bytes_std {
    #![allow(clippy::type_complexity)]
    use azure_core::base64;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::result::Result;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let to_deserialize = <Option<Vec<String>>>::deserialize(deserializer)?;
        match to_deserialize {
            Some(to_deserialize) => {
                let mut decoded0 = <Vec<Vec<u8>>>::new();
                for v in to_deserialize {
                    decoded0.push(base64::decode(v).map_err(serde::de::Error::custom)?);
                }
                Ok(decoded0)
            }
            None => Ok(<Vec<Vec<u8>>>::default()),
        }
    }

    pub fn serialize<S>(to_serialize: &[Vec<u8>], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded0 = to_serialize.iter().map(base64::encode).collect();
        <Vec<String>>::serialize(&encoded0, serializer)
    }
}
