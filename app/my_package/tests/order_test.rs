use project_my_package::{Category, Customer, Order, Product};

#[test]
fn test_total_bull_without_discount() {
    let product = Product::new(1, String::from("Book"), 19.9, Category::Books);
    let customer = Customer::new(1, String::from("Bob"), String::from("bob@example.com"));
    let order = Order::new(2, product, customer, 3);

    assert_eq!(format!("{:.2}", order.total_bill()), "65.67");
}

#[test]
fn test_total_bull_with_discount() {
    let product = Product::new(1, String::from("Book"), 19.99, Category::Books);
    let customer = Customer::new(1, String::from("Bob"), String::from("bob@example.com"));
    let order = Order::new(2, product, customer, 10);

    assert_eq!(format!("{:.2}", order.total_bill()), "197.90");
}
