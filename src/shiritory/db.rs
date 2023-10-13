use std::sync::Arc;

use tokio_postgres::Client;

pub(crate) async fn get_last(client: &Client) -> String {
    let res = client
        .query("select id, value from word order_by id desc limit 1", &[])
        .await
        .unwrap();
    res[0].get(1)
}

pub(crate) async fn add_word(word: &str, client: &Client) {
    client
        .query("insert into word values $1", &[&word])
        .await
        .unwrap();
}

pub(crate) async fn create_word_db(name: &str, client: &Client) {
    client.query("create table $1 (id SERIAL, value text)", &[&name]).await.unwrap();
}
