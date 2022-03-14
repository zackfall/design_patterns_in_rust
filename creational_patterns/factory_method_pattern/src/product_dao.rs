use crate::adapters::db_adapter::DBAdapter;
use crate::db_factory::DBFactory;
use crate::product::Product;
use crate::utils::connection::DBConnection;
use postgres;
use rusqlite::{self, params};

pub struct ProductDAO {
    db_adapter: Box<dyn DBAdapter>,
}

impl ProductDAO {
    pub fn new() -> Self {
        Self {
            db_adapter: DBFactory::get_default_db_adapter(),
        }
    }

    pub fn find_all_products(&self) -> Vec<Product> {
        let connection: DBConnection = self.db_adapter.get_connection();
        let mut product_list: Vec<Product> = Vec::new();
        let query = "SELECT id_product, product_name, product_price FROM products";
        match connection {
            DBConnection::PSQLConn(mut client) => {
                let statement: postgres::Statement = match client.prepare(query) {
                    Ok(stmt) => stmt,
                    Err(err) => panic!("{}", err),
                };
                let rows = match client.query(&statement, &[]) {
                    Ok(rows) => rows,
                    Err(err) => panic!("{}", err),
                };

                for row in rows {
                    product_list.push(Product::new(
                        row.get("id_product"),
                        row.get("product_name"),
                        row.get("product_price"),
                    ))
                }
            }
            DBConnection::SQLiteConn(conn) => {
                let mut statement: rusqlite::Statement = match conn.prepare(query) {
                    Ok(stmt) => stmt,
                    Err(err) => panic!("{}", err),
                };
                let mut rows = match statement.query([]) {
                    Ok(rows) => rows,
                    Err(err) => panic!("{}", err),
                };

                while let Some(row) = match rows.next() {
                    Ok(row) => row,
                    Err(err) => panic!("{}", err),
                } {
                    product_list.push(Product::new(
                        row.get_unwrap("id_product"),
                        row.get_unwrap("product_name"),
                        row.get_unwrap("product_price"),
                    ))
                }
            }
        }
        product_list
    }

    pub fn save_product(&self, product: Product) -> bool {
        let connection: DBConnection = self.db_adapter.get_connection();
        match connection {
            DBConnection::PSQLConn(mut client) => {
                match client.execute(
                    "INSERT INTO products(id_product, product_name, product_price) VALUES ($1, $2, $3)",
                    &[&product.get_id_product(), &product.get_product_name(), &product.get_price()]
                ) {
                    Ok(_) => true,
                    Err(err) => {
                        println!("{}", err);
                        false
                    },
                }
            }
            DBConnection::SQLiteConn(conn) => {
                match conn.execute(
                    "INSERT INTO products(id_product, product_name, product_price) VALUES (?1, ?2, ?3)",
                    params![product.get_id_product(), product.get_product_name(), product.get_price()],
                ) {
                    Ok(_) => true,
                    Err(err) => {
                        println!("{}", err);
                        false
                    },
                }
            }
        }
    }
}
