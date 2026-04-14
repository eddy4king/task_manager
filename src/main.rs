mod db;

use dotenv::dotenv;


#[tokio::main]
async fn main() {
    dotenv().ok();
    db::connect().await;
}
    
