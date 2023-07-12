use url::Url;
use libsql_client::Client;

pub async fn get_db() -> Client {
    // Initialize database
    let db = Client::from_config(libsql_client::Config {
        url: Url::parse("http://127.0.0.1:8080").unwrap(),
        auth_token: None,
    })
    .await
    .unwrap();

    return db;
}

pub async fn init_database(db: Client){
    let result = db.execute("CREATE TABLE IF NOT EXISTS 'file_uploads' ('id' varchar,'token' varchar, 'downloaded' integer NOT NULL DEFAULT '0', PRIMARY KEY (id));").await;
    println!("{:?}", result);
}