use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    event_id: Option<ObjectId>,
    title: String,
    description: String,
    time: DateTime,
    location: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,
}
