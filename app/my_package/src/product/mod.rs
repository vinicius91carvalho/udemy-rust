pub use category::Category;

/// Struct for storing product related information.
#[derive(PartialEq, Debug)]
pub struct Product {
    id: u64,
    pub name: String,
    price: f64,
    category: Category,
}

mod category;

impl Product {
    fn calculate_tax(&self) -> f64 {
        self.price * 0.1
    }

    pub fn product_price(&self) -> f64 {
        self.price + self.calculate_tax()
    }

    /// # Example
    /// ```
    /// use project_my_package::Category;
    /// use project_my_package::Product;
    /// let some_product = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);
    /// assert_eq!(some_product.name, String::from("Laptop"));
    /// ```
    pub fn new(id: u64, name: String, price: f64, category: Category) -> Product {
        Product {
            id,
            name,
            price,
            category,
        }
    }
}
