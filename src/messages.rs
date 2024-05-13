use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Request<T> {
    pub seq: u64,
    #[serde(rename = "type")]
    pub type_: RequestTag,
    pub command: String,
    #[serde(default)]
    pub body: T,
}

#[derive(Deserialize, Serialize)]
pub struct Event<T> {
    pub seq: u64,
    #[serde(rename = "type")]
    pub type_: EventTag,
    pub event: String,
    #[serde(default)]
    pub body: T,
}

#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub seq: u64,
    #[serde(rename = "type")]
    pub type_: ResponseTag,
    pub request_seq: u64,
    pub success: bool,
    pub command: String,
    #[serde(default)]
    pub body: T,
}

macro_rules! tag {
    (pub struct $name:ident => $impl_name:ident $impl_string:tt $tag:tt;) => {
        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
        #[serde(from = $impl_string)]
        pub struct $name;

        impl From<$impl_name> for $name {
            fn from(_: $impl_name) -> Self {
                $name
            }
        }

        impl From<$name> for $impl_name {
            fn from(_: $name) -> Self {
                $impl_name::Tag
            }
        }

        #[derive(Deserialize, Serialize)]
        enum $impl_name {
            #[serde(rename = $tag)]
            Tag,
        }
    };
}

tag!(pub struct RequestTag => RequestTagImpl "RequestTagImpl" "request";);
tag!(pub struct ResponseTag => ResponseTagImpl "ResponseTagImpl" "response";);
tag!(pub struct EventTag => EventTagImpl "EventTagImpl" "event";);
