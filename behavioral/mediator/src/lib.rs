use std::cell::RefCell;

pub struct Mediator {
    sellers: Vec<RefCell<Seller>>,
}

impl Mediator {
    pub fn new() -> Self {
        Self { sellers: vec![] }
    }

    pub fn add_sellers(&mut self, sellers: Vec<Seller>) {
        let mut sellers = sellers
            .into_iter()
            .map(|seller| RefCell::new(seller))
            .collect();

        self.sellers.append(&mut sellers);
    }

    pub fn buy(&mut self, product_id: &str) -> Option<SellerProduct> {
        let product = self
            .sellers
            .iter()
            .find_map(|seller| seller.borrow_mut().sell(product_id));

        println!("Produto comprado: {:?}", product);

        product
    }

    pub fn show_products(&self) {
        self.sellers
            .iter()
            .for_each(|seller| seller.borrow().show_products());
    }

    /// Get a reference to the mediator's sellers.
    pub fn sellers(&self) -> &[RefCell<Seller>] {
        self.sellers.as_ref()
    }
}

pub struct Seller {
    products: Vec<SellerProduct>,
}

impl Seller {
    pub fn new() -> Self {
        Self { products: vec![] }
    }

    pub fn show_products(&self) {
        self.products
            .iter()
            .for_each(|product| println!("{:?}", product));
    }

    pub fn add_products(&mut self, products: Vec<SellerProduct>) {
        let mut products = products;
        self.products.append(&mut products);
    }

    pub fn sell(&mut self, product_id: &str) -> Option<SellerProduct> {
        let position = self
            .products
            .iter()
            .position(|product| product.id() == product_id);

        let index = match position {
            Some(pos) => pos,
            None => return None,
        };

        let product = self.products.remove(index);

        Some(product)
    }
}

#[derive(Debug)]
pub struct SellerProduct {
    id: String,
    name: String,
    price: f32,
}

impl SellerProduct {
    pub fn new(id: &str, name: &str, price: f32) -> Self {
        let id = String::from(id);
        let name = String::from(name);

        Self { id, name, price }
    }

    /// Get a reference to the seller product's id.
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    /// Get a reference to the seller product's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the seller product's price.
    pub fn price(&self) -> f32 {
        self.price
    }
}

pub struct Buyer {
    mediator: Mediator,
}

impl Buyer {
    pub fn new(mediator: Mediator) -> Self {
        Self { mediator }
    }

    pub fn view_products(&self) {
        self.mediator.show_products();
    }

    pub fn buy(&mut self, product_id: &str) -> Option<SellerProduct> {
        self.mediator.buy(product_id)
    }
}
