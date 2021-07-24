use std::fmt;

#[derive(Debug)]
pub struct Product {
    id_product: i32,
    product_name: String,
    price: f32,
}

impl Product {
    pub fn new(id_product: i32, product_name: String, price: f32) -> Self {
        Self {
            id_product,
            product_name,
            price,
        }
    }

    pub fn get_id_product(&self) -> i32 {
        self.id_product
    }

    pub fn set_id_product(&mut self, id_product: i32) {
        self.id_product = id_product;
    }

    pub fn get_product_name(&self) -> String {
        self.product_name.clone()
    }

    pub fn set_product_name(&mut self, product_name: &str) {
        self.product_name = product_name.to_string();
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }

    pub fn set_price(&mut self, price: f32) {
        self.price = price;
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Product {{ id_product = {}, product_name = {}, price = {} }}",
            self.id_product, self.product_name, self.price
        )
    }
}
