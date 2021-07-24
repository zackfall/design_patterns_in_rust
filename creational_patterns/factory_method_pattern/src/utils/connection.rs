use postgres::Client;
use rusqlite::Connection;

pub enum DBConnection {
    PSQLConn(Client),
    SQLiteConn(Connection),
}
