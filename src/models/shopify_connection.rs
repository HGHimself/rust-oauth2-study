use crate::schema::shopify_connections;
use crate::utils::now;
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "shopify_connections"]
pub struct ShopifyConnection {
    pub id: i32,
    pub shop: String,
    pub nonce: String,
    pub access_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub active: bool,
}

#[derive(Insertable)]
#[table_name = "shopify_connections"]
pub struct NewShopifyConnection {
    pub shop: String,
    pub nonce: String,
    pub access_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub active: bool,
}

impl NewShopifyConnection {
    pub fn new(shop: String, nonce: String) -> Self {
        NewShopifyConnection {
            shop,
            nonce,
            access_token: None,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
            active: true,
        }
    }

    pub fn insert(&self, conn: &PgConnection) -> ShopifyConnection {
        create(conn, self)
    }
}

pub fn create(
    conn: &PgConnection,
    new_shopify_connection: &NewShopifyConnection,
) -> ShopifyConnection {
    diesel::insert_into(shopify_connections::table)
        .values(new_shopify_connection)
        .get_result(conn)
        .expect("Error saving new shopify_connection")
}

pub fn read(conn: &PgConnection) -> Vec<ShopifyConnection> {
    shopify_connections::table
        .load::<ShopifyConnection>(conn)
        .expect("Error loading shopify_connection")
}

pub fn read_by_shop(conn: &PgConnection, shop: String) -> Vec<ShopifyConnection> {
    shopify_connections::table
        .filter(shopify_connections::shop.eq(shop))
        .load::<ShopifyConnection>(conn)
        .expect("Error loading shopify_connection")
}

pub fn read_by_shop_and_nonce(
    conn: &PgConnection,
    shop: String,
    nonce: String,
) -> Vec<ShopifyConnection> {
    shopify_connections::table
        .filter(shopify_connections::shop.eq(shop))
        .filter(shopify_connections::nonce.eq(nonce))
        .load::<ShopifyConnection>(conn)
        .expect("Error loading shopify_connection")
}

pub fn update_access_token(
    conn: &PgConnection,
    shopify_connection: &ShopifyConnection,
    access_token: String,
) -> QueryResult<usize> {
    diesel::update(shopify_connection)
        .set((
            shopify_connections::access_token.eq(access_token),
            shopify_connections::updated_at.eq(now()),
        ))
        .execute(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::establish_connection_test;

    fn cleanup_table(conn: &PgConnection) {
        diesel::delete(shopify_connections::table)
            .execute(conn)
            .unwrap();
    }

    fn mock_struct() -> NewShopifyConnection {
        NewShopifyConnection::new(
            String::from("ShopName"),
            String::from("00a329c0648769a73afac7f9381e08fb43dbea72"),
        )
    }

    #[test]
    fn it_creates_a_shopify_connection() {
        let conn = establish_connection_test();

        create(&conn, &mock_struct());

        let shopify_connection = shopify_connections::table
            .load::<ShopifyConnection>(&conn)
            .expect("Error loading shopify_connection");

        assert_eq!(1, shopify_connection.len());

        cleanup_table(&conn);
    }

    #[test]
    fn it_reads_a_shopify_connection() {
        let conn = establish_connection_test();

        let new_shopify_connection = mock_struct();

        let created_shopify_connection = diesel::insert_into(shopify_connections::table)
            .values(&new_shopify_connection)
            .get_result::<ShopifyConnection>(&conn)
            .expect("Error saving new shopify_connection");

        let shopify_connection = read(&conn);

        assert!(0 < shopify_connection.len());

        let my_shopify_connection = shopify_connection
            .iter()
            .find(|&x| x.shop == new_shopify_connection.shop);
        assert!(
            my_shopify_connection.is_some(),
            "Could not find the created shopify_connection in the database!"
        );

        cleanup_table(&conn);
    }

    #[test]
    fn it_reads_a_shopify_connection_by_shop() {
        let conn = establish_connection_test();
        let shop = String::from("ShopNameBaby");

        // make 2 shopify_connections, each with different categories
        let mut new_shopify_connection = mock_struct();
        create(&conn, &new_shopify_connection);

        new_shopify_connection.shop = shop.clone();
        create(&conn, &new_shopify_connection);

        let shopify_connection = read_by_shop(&conn, shop.clone());

        assert_eq!(1, shopify_connection.len());

        let my_shopify_connection = shopify_connection.iter().find(|x| x.shop == shop);
        assert!(
            my_shopify_connection.is_some(),
            "Could not find the created shopify_connection in the database!"
        );

        cleanup_table(&conn);
    }

    #[test]
    fn it_reads_a_shopify_connection_by_shop_and_nonce() {
        let conn = establish_connection_test();
        let nonce =
            String::from("0cd1136c6702de4410d06d3ae80f592c9b2132ea232011bcc78fb53862cbd9ee");

        // make 2 shopify_connections, each with different categories
        let mut new_shopify_connection = mock_struct();
        create(&conn, &new_shopify_connection);

        new_shopify_connection.nonce = nonce.clone();
        create(&conn, &new_shopify_connection);

        let shopify_connection =
            read_by_shop_and_nonce(&conn, String::from("ShopName"), nonce.clone());

        assert_eq!(1, shopify_connection.len());

        let my_shopify_connection = shopify_connection.iter().find(|x| x.nonce == nonce);
        assert!(
            my_shopify_connection.is_some(),
            "Could not find the created shopify_connection in the database!"
        );

        cleanup_table(&conn);
    }

    #[test]
    fn it_updates_a_shopify_connection_access_token() {
        let conn = establish_connection_test();

        let shopify_connection = create(&conn, &mock_struct());
        let access_token = String::from("super ssssecret");

        update_access_token(&conn, &shopify_connection, access_token.clone());

        let shopify_connections = read_by_shop(&conn, shopify_connection.shop);

        assert_eq!(1, shopify_connections.len());
        let my_shopify_connection = shopify_connections
            .iter()
            .find(|x| x.access_token.as_ref().unwrap() == &access_token);
        assert!(
            my_shopify_connection.is_some(),
            "Could not find the created shopify_connection in the database!"
        );

        cleanup_table(&conn);
    }
}
