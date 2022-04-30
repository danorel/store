#[derive(Debug)]
pub struct Client {
    balance: u32,
    shopping_cart: Vec<(String, u8, u32)>
}

impl Client {
    pub fn new(balance: u32) -> Self {
        Client { balance, shopping_cart: Vec::new() }
    }

    pub fn shopping_cart(&mut self) -> &mut Vec<(String, u8, u32)> {
        &mut self.shopping_cart
    }

    pub fn balance(&self) -> &u32 {
        &self.balance
    }

    pub fn spend(&mut self, money: u32) -> () {
        self.balance -= money;
    }
}