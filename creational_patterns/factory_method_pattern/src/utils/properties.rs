use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Properties {
    pub db_factory: DBFactoryProperties,
    pub db_psql: DBPostgreSQLProperties,
    pub db_sqlite: DBSQLiteProperties,
    pub db_mariadb: MariaDBProperties,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DBFactoryProperties {
    pub default_db_struct: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DBPostgreSQLProperties {
    pub host: String,
    pub port: String,
    pub db_name: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DBSQLiteProperties {
    pub db_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MariaDBProperties {
    pub host: String,
    pub port: String,
    pub db_name: String,
    pub user: String,
    pub password: String,
}

impl Properties {
    pub fn get_properties() -> Properties {
        let input_file = "./properties.toml";
        let mut file = File::open(input_file).unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();
        let properties: Properties = toml::from_str(&file_content).unwrap();

        properties
    }
}
