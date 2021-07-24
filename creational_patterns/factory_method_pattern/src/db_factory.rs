use crate::adapters::{
    db_adapter::DBAdapter, psql_db_adapter::PostgreSQLDBAdapter, sqlite_db_adapter::SQLiteDBAdapter,
};
use crate::db_type::DBType;
use crate::utils::properties::Properties;

pub struct DBFactory;

impl DBFactory {
    pub fn get_db_adapter(db_type: DBType) -> Box<dyn DBAdapter> {
        match db_type {
            DBType::PostgreSQL => Box::new(PostgreSQLDBAdapter::new()),
            DBType::SQLite => Box::new(SQLiteDBAdapter::new()),
        }
    }

    pub fn get_default_db_adapter() -> Box<dyn DBAdapter> {
        let default_db = Properties::get_properties().db_factory.default_db_struct;
        match default_db.as_str() {
            "PostgreSQL" => {
                println!("Default DB ==> {}", default_db);
                Box::new(PostgreSQLDBAdapter::new())
            }
            "SQLite" => {
                println!("Default DB ==> {}", default_db);
                Box::new(SQLiteDBAdapter::new())
            }
            _ => {
                panic!(
                    "The database that you passed in the properties toml file, is not supported"
                );
            }
        }
    }
}
