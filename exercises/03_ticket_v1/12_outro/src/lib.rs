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
macro_rules! empty_string_not_allowd {
    ($value: expr, $field_name: expr) => {
        if $value.is_empty() {
            panic!("{} should not be empty", $field_name);
        }
    };
}

macro_rules! string_length_control {
    ($value: expr, $string_length: expr, $field_name: expr) => {
        if $value.len() > $string_length {
            panic!("{} has length greater than {}", $field_name, $string_length)
        }
    };
}

macro_rules! non_zero_validation {
    ($value: expr, $field_name: expr) => {
        if $value == 0 {
            panic!("For {}, zero value should not be allowed", $field_name);
        }
    };
}

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        empty_string_not_allowd!(product_name, "Product Name");
        string_length_control!(product_name, 300, "Product Name");
        non_zero_validation!(quantity, "quantity");
        non_zero_validation!(unit_price, "unit_price");
        Self {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn total(&self) -> u32 {
        self.unit_price.saturating_mul(self.quantity)
    }

    pub fn set_product_name(&mut self, product_name: String) {
        empty_string_not_allowd!(product_name, "Product Name");
        string_length_control!(product_name, 300, "Product Name");
        self.product_name = product_name;
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        non_zero_validation!(quantity, "quantity");
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: u32) {
        non_zero_validation!(unit_price, "unit_price");
        self.unit_price = unit_price;
    }
}
