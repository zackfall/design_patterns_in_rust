use crate::utils::connection::DBConnection;

/// This trait define the structure of the products that the Factory can create.
/// In this case we have two concrete connections, one for PostgreSQL and other for SQLite.
pub trait DBAdapter {
    /// This method create connections to the database.
    fn get_connection(&self) -> DBConnection;
}
