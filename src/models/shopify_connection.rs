pub struct ShopifyConnection {
    pub shop: String,
    pub hmac: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub active: bool,
    pub accessToken: Option<String>
}
