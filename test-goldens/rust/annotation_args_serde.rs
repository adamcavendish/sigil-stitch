#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
}
