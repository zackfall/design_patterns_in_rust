use crate::adapters::db_adapter::DBAdapter;
use crate::utils::connection::DBConnection;
use crate::utils::properties::Properties;
use rusqlite::Connection;

pub struct SQLiteDBAdapter {
    db_name: String,
}

impl SQLiteDBAdapter {
    pub fn new() -> Self {
        let properties = Properties::get_properties();
        Self {
            db_name: properties.db_sqlite.db_name,
        }
    }
}

impl DBAdapter for SQLiteDBAdapter {
    fn get_connection(&self) -> DBConnection {
        let client = Connection::open(self.db_name.as_str());
        match client {
            Ok(client) => {
                println!("Connection DB ==> {}", self.db_name);
                DBConnection::SQLiteConn(client)
            }
            Err(error) => panic!("{}", error),
        }
    }
}
