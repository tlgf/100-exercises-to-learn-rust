// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    fn check_product_name(prod_name: &String) -> () {
        if prod_name.is_empty() || prod_name.len() > 300 {
            panic!();
        }
    }

    fn check_quantity(qty: u32) -> () {
        if qty <= 0 {
            panic!();
        }
    }

    fn check_unit_price(price: u32) -> () {
        if price <= 0 {
            panic!();
        }
    }

    pub fn set_product_name(&mut self, prod_name: String) -> () {
        Self::check_product_name(&prod_name);
        self.product_name = prod_name;
    }

    pub fn set_quantity(&mut self, qty: u32) -> () {
        Self::check_quantity(qty);
        self.quantity = qty;
    }

    pub fn set_unit_price(&mut self, price: u32) -> () {
        Self::check_unit_price(price);
        self.unit_price = price;
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn new(prod_name: String, qty: u32, price: u32) -> Order {
        Self::check_product_name(&prod_name);
        Self::check_quantity(qty);
        Self::check_unit_price(price);

        Order {
            product_name: prod_name,
            quantity: qty,
            unit_price: price,
        }
    }
}
