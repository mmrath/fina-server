
#[derive(
Queryable, Identifiable, AsChangeset, Associations, Debug, Serialize, Deserialize, Clone,Eq,PartialEq
)]
#[table_name = "message"]
pub struct Message {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub subject: String,
    pub message_type: MessageType,
    pub body: Vec<u8>,
    pub body_type: BodyType,
}

#[derive(Insertable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "message"]
pub struct NewMessage<'a> {
    pub created_at: DateTime<Utc>,
    pub subject: String,
    pub message_type: MessageType,
    pub body: Vec<u8>,
    pub body_type: BodyType,
}

#[derive(
Debug, Serialize, Deserialize, Clone,Copy, Eq,PartialEq
)]
pub enum BodyType {
    PlainText,
    Html
}

#[derive(
Debug, Serialize, Deserialize, Clone,Copy, Eq,PartialEq
)]
pub enum MessageType {
    Email
}


#[derive(
Queryable, Identifiable, AsChangeset, Associations, Debug, Serialize, Deserialize, Clone,Eq,PartialEq
)]
#[table_name = "message_address"]
pub struct MessageAddress {
    pub id: i64,
    pub message_id: i64,
    pub address_type: AddressType,
    pub name: Option<String>,
    pub address: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "message_address"]
pub struct NewMessageAddress {
    pub message_id: i64,
    pub address_type: AddressType,
    pub name: Option<String>,
    pub address: String,

}


#[derive(
    Debug, Serialize, Deserialize, Clone,Copy, Eq,PartialEq
)]
pub enum AddressType {
    From,
    To,
    Cc,
    Bcc
}

#[derive(
Queryable, Identifiable, AsChangeset, Associations, Debug, Serialize, Deserialize, Clone,Eq,PartialEq
)]
#[table_name = "message_attachment"]
pub struct MessageAttachment {
    pub id: i64,
    pub message_id: i64,
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Insertable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "message_attachment"]
pub struct NewMessageAttachment {
    pub id: i64,
    pub message_id: i64,
    pub name: String,
    pub data: Vec<u8>,
}

