use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[must_use] pub fn establish_connection_pool() -> PgPool {
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let manager = ConnectionManager::<PgConnection>::new(database_url);
	Pool::builder()
		.max_size(15)
		.test_on_check_out(true)
		.build(manager)
		.expect("Failed to create connection pool")
}
