use crate::adapters::db_adapter::DBAdapter;
use crate::utils::connection::DBConnection;
use crate::utils::properties::Properties;
use mysql::{Opts, Pool};

pub struct MariaDBAdapter {
    host: String,
    port: String,
    db_name: String,
    user: String,
    password: String,
}

impl MariaDBAdapter {
    pub fn new() -> Self {
        let properties = Properties::get_properties();
        Self {
            host: properties.db_mariadb.host,
            port: properties.db_mariadb.port,
            db_name: properties.db_mariadb.db_name,
            user: properties.db_mariadb.user,
            password: properties.db_mariadb.password,
        }
    }

    pub fn create_connection_url(&self) -> String {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db_name
        );
        url
    }
}

impl DBAdapter for MariaDBAdapter {
    fn get_connection(&self) -> DBConnection {
        let url = self.create_connection_url();
        match Pool::new(Opts::from_url(url.as_str()).unwrap()) {
            Ok(pool) => match pool.get_conn() {
                Ok(pconn) => {
                    println!("Connection string ==> {}", url);
                    DBConnection::MariaDBConn(pconn)
                }
                Err(err) => panic!("{}", err),
            },
            Err(err) => panic!("{}", err),
        }
    }
}
