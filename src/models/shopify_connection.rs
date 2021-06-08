use crate::schema::shopifyConnection;
use crate::utils::now;
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use std::error::Error;
use std::time;

#[derive(Queryable, Debug)]
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
#[table_name = "shopifyConnection"]
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
    diesel::insert_into(shopifyConnection::table)
        .values(new_shopify_connection)
        .get_result(conn)
        .expect("Error saving new shopify_connection")
}

pub fn read(conn: &PgConnection) -> Vec<ShopifyConnection> {
    shopifyConnection::table
        .load::<ShopifyConnection>(conn)
        .expect("Error loading shopify_connection")
}

pub fn read_by_shop(conn: &PgConnection, shop: String) -> Vec<ShopifyConnection> {
    shopifyConnection::table
        .filter(shopifyConnection::shop.eq(shop))
        .load::<ShopifyConnection>(conn)
        .expect("Error loading shopify_connection")
}

pub fn read_by_shop_and_nonce(
    conn: &PgConnection,
    shop: String,
    nonce: String,
) -> Vec<ShopifyConnection> {
    shopifyConnection::table
        .filter(shopifyConnection::shop.eq(shop))
        .filter(shopifyConnection::nonce.eq(nonce))
        .load::<ShopifyConnection>(conn)
        .expect("Error loading shopify_connection")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::establish_connection_test;

    fn cleanup_table(conn: &PgConnection) {
        diesel::delete(shopifyConnection::table)
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

        let shopify_connection = shopifyConnection::table
            .load::<ShopifyConnection>(&conn)
            .expect("Error loading shopify_connection");

        assert_eq!(1, shopify_connection.len());

        cleanup_table(&conn);
    }

    #[test]
    fn it_reads_a_shopify_connection() {
        let conn = establish_connection_test();

        let new_shopify_connection = mock_struct();

        let created_shopify_connection = diesel::insert_into(shopifyConnection::table)
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
}
