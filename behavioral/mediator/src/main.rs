use mediator::{Buyer, Mediator, Seller, SellerProduct};

pub fn main() {
    let mut mediator = Mediator::new();

    let mut seller1 = Seller::new();
    seller1.add_products(vec![
        SellerProduct::new("1", "Caneta", 8.9),
        SellerProduct::new("2", "Calça", 37.9),
    ]);

    let mut seller2 = Seller::new();
    seller2.add_products(vec![
        SellerProduct::new("3", "Carro", 49_000_f32),
        SellerProduct::new("4", "Lápis", 2.4),
    ]);

    mediator.add_sellers(vec![seller1, seller2]);

    let mut buyer = Buyer::new(mediator);
    buyer.view_products();
    buyer.buy("2");
    buyer.buy("3");
    buyer.view_products();
}
