use crate::adapters::db_adapter::DBAdapter;
use crate::utils::connection::DBConnection;
use crate::utils::properties::Properties;
use postgres::{Client, NoTls};

pub struct PostgreSQLDBAdapter {
    host: String,
    port: String,
    db_name: String,
    user: String,
    password: String,
}

impl PostgreSQLDBAdapter {
    pub fn new() -> Self {
        let properties = Properties::get_properties();
        Self {
            host: properties.db_psql.host,
            port: properties.db_psql.port,
            db_name: properties.db_psql.db_name,
            user: properties.db_psql.user,
            password: properties.db_psql.password,
        }
    }

    pub fn create_connection_url(&self) -> String {
        let url = format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db_name
        );
        url
    }
}

impl DBAdapter for PostgreSQLDBAdapter {
    fn get_connection(&self) -> DBConnection {
        let url = self.create_connection_url();
        match Client::connect(url.as_str(), NoTls) {
            Ok(client) => {
                println!("Connection string ==> {}", url);
                DBConnection::PSQLConn(client)
            }
            Err(error) => panic!("{}", error),
        }
    }
}
