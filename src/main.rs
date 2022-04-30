mod entities;

pub fn main () {
    let mut shop = entities::shop::Shop::new(
        vec![
            "milk".to_string(),
            "cereal".to_string()
        ], 
        vec![
            (10, 1500),
            (10, 2500)
        ]);
    println! ("${:?}", shop);

    let mut client = entities::client::Client::new(10000);
    println! ("${:?}", client);

    shop.buy(&mut client, &"milk".to_string(), 5).unwrap();
    shop.buy(&mut client, &"cereal".to_string(), 1).unwrap();
    println!(
        "Client's card after putting products there: {:?}",
        &client.shopping_cart()
    );

    shop.checkout(&mut client).unwrap();
    println!(
        "Client's card after the checkout: {:?}",
        &client.shopping_cart()
    );

    assert_eq!(*client.balance(), 0);
    assert_eq!(client.shopping_cart().len(), 0);

    println!("The shop closes!: {:?}", &shop);
}