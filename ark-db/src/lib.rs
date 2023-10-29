pub mod schema;

use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use dotenvy::dotenv;

pub type DBConn<'a> = bb8::PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type DBPool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub fn url() -> String {
    dotenv().ok();

    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub async fn get_pool() -> DBPool {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url());

    bb8::Pool::builder().build(manager).await.unwrap()
}
