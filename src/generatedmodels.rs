#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;

use serde::ser::Serializer;

use std::collections::HashMap;
use swagger;
use std::string::ParseError;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct Error {
    #[serde(rename = "message")]
    pub message: String,

}

impl Error {
    pub fn new(message: String, ) -> Error {
        Error {
            message: message,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct KeyData {
    #[serde(rename = "key_name")]
    pub key_name: String,

    #[serde(rename = "key")]
    pub key: swagger::ByteArray,

}

impl KeyData {
    pub fn new(key_name: String, key: swagger::ByteArray, ) -> KeyData {
        KeyData {
            key_name: key_name,
            key: key,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct KeyRequest {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "private")]
    pub private: String,

}

impl KeyRequest {
    pub fn new(id: String, private: String, ) -> KeyRequest {
        KeyRequest {
            id: id,
            private: private,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct KeyRequestContext {
    #[serde(rename = "suggested_key_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub suggested_key_name: Option<String>,

}

impl KeyRequestContext {
    pub fn new() -> KeyRequestContext {
        KeyRequestContext {
            suggested_key_name: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct KeyRequestId(String);

impl ::std::convert::From<String> for KeyRequestId {
    fn from(x: String) -> Self {
        KeyRequestId(x)
    }
}

impl std::str::FromStr for KeyRequestId {
    type Err = ParseError;
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        Ok(KeyRequestId(x.to_string()))
    }
}

impl ::std::convert::From<KeyRequestId> for String {
    fn from(x: KeyRequestId) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for KeyRequestId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::ops::DerefMut for KeyRequestId {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


