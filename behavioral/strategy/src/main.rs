use strategy::*;

pub fn main() {
    let mut shopping_cart = ECommerceShoppingCart::new();
    shopping_cart.set_discount_strategy(Box::new(DefaultDiscountStrategy));

    shopping_cart.add_products(vec![
        ECommerceProduct::new("P1", 50_f32),
        ECommerceProduct::new("P2", 50_f32),
    ]);

    println!("{}", shopping_cart.get_total_with_discount());

    shopping_cart.set_discount_strategy(Box::new(NewDiscountStrategy));
    println!("{}", shopping_cart.get_total_with_discount());
}
