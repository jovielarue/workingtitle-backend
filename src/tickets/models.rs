use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ticket {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    ticket_id: Option<ObjectId>,
    user_id: ObjectId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialTicket {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub ticket_id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<ObjectId>,
}
