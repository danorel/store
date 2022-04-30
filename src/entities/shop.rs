use std::collections::HashMap;

use super::client::Client;

#[derive(Debug)]
pub struct Shop {
    store: HashMap<String, (u8, u32)>
}

impl Shop {
    pub fn new(names: Vec<String>, descriptions: Vec<(u8, u32)>) -> Self {
        Shop { store: names.into_iter().zip(descriptions.into_iter()).collect() }
    }

    pub fn buy(&mut self, client: &mut Client, product_name: &str, quantity: u8) -> Result<(), String> {
        if !self.store.contains_key(product_name) {
            return Err(format!("Product {product_name} doesn't exist in Shop"))
        }
        let &(rest, price) = self.store.get(product_name).unwrap();
        if quantity > rest {
            return Err(format!("Product {product_name} request quantity exceeds Shop's limits"))
        }
        client.shopping_cart().push((product_name.to_string(), quantity, price));
        Ok(())
    }

    pub fn checkout(&mut self, client: &mut Client) -> Result<(), String> {
        let mut total: u32 = 0;
        for shopping_item in client.shopping_cart().iter_mut() {
            let (_, quantity, price) = *shopping_item;
            total += (quantity as u32) * price;
        }
        if total > *client.balance() {
            return Err(format!("Money is not enought"))
        }
        client.spend(total);
        for shopping_item in client.shopping_cart().iter_mut() {
            let (product_name, quantity, _) = shopping_item;
            self.move_out(product_name.to_string(), *quantity);
        }
        client.shopping_cart().clear();
        Ok(())
    }

    fn move_out(&mut self, product_name: String, quantity: u8) -> Result<(), String> {
        if !self.store.contains_key(&product_name) {
            return Err(format!("Cannot move out {product_name}"));
        }
        let product = self.store.get_mut(&product_name).unwrap();
        product.0 -= quantity;
        Ok(())
    }
}