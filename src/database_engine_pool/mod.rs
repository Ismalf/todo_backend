use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};



pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

fn init(database_url: &str) -> Result<MysqlPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn connect() -> MysqlPool {
    println!("connecting");
    init("mysql://root:1234@127.0.0.1:3306/todo").expect("Error")
}

pub fn get_conn(pool: &MysqlPool) -> Result<MySqlPooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get connection")
}
