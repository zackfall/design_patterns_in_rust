// Example:
// Imaginemos un escenario donde deseamos tener la opción de conectarnos a dos bases de datos
// distintas, como SQLite y PostgreSQL, esto podría darse por la necesidad de darle a los usuarios de
// nuestra aplicación la posibilidad de tener una base de datos robusta como PostgreSQL, o una opción
// más ligera como SQLite. Sea cual sea el motivo por el cual el usuario decide utilizar una base
// de datos, nosotros tenemos que tener los mecanismos para soportarla.
//
// Para esto desarrollaremos una estructura de acceso a datos de productos que nos permite guardar
// productos y consultarlos, el principal objetivo es que el cliente pueda utilizar la misma
// estructura sin la necesidad de cambiar de acceso dependiendo la base de datos a utilizar.
use factory_method_pattern::product::Product;
use factory_method_pattern::product_dao::ProductDAO;

fn main() {
    // creamos productos para registrar
    let product_a: Product = Product::new(5, "Better PC".to_string(), 500.55);
    let product_b: Product = Product::new(6, "Ps5 with some games".to_string(), 800.0);

    // Creamos una instancia del DAO
    let product_dao: ProductDAO = ProductDAO::new();

    // Guardamos los productos
    product_dao.save_product(product_a);
    product_dao.save_product(product_b);

    // consultamos nuevamente los productos
    let products: Vec<Product> = product_dao.find_all_products();
    println!("Product size ==> {}", products.len());
    for product in products {
        println!("{}", product);
    }
}
