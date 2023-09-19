use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use dotenvy::dotenv;

pub type DBConn = bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type DBPool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub struct DB {
    pub pool: DBPool,
}

impl DB {
    pub fn url() -> String {
        dotenv().ok();

        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    }

    pub async fn new() -> DB {
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(Self::url());
        let pool = bb8::Pool::builder().build(manager).await.unwrap();

        Self { pool }
    }
}
