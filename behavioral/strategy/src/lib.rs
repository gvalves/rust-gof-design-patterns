pub trait DiscountStrategy {
    fn get_discount(&self, cart: &ECommerceShoppingCart) -> f32;
}

pub struct ECommerceShoppingCart {
    products: Vec<ECommerceProduct>,
    discount_strategy: Box<dyn DiscountStrategy>,
}

impl ECommerceShoppingCart {
    pub fn new() -> Self {
        Self {
            products: vec![],
            discount_strategy: Box::new(NoDiscountStrategy),
        }
    }

    pub fn add_products(&mut self, products: Vec<ECommerceProduct>) {
        let mut products = products;
        self.products.append(&mut products);
    }

    pub fn get_total(&self) -> f32 {
        self.products
            .iter()
            .fold(0f32, |sum, product| sum + product.price())
    }

    pub fn get_total_with_discount(&self) -> f32 {
        self.discount_strategy.get_discount(self)
    }

    /// Get a reference to the ecommerce shopping cart's products.
    pub fn products(&self) -> &[ECommerceProduct] {
        self.products.as_ref()
    }

    /// Set the ecommerce shopping cart's discount strategy.
    pub fn set_discount_strategy(&mut self, discount_strategy: Box<dyn DiscountStrategy>) {
        self.discount_strategy = discount_strategy;
    }
}

pub struct NoDiscountStrategy;

impl DiscountStrategy for NoDiscountStrategy {
    fn get_discount(&self, cart: &ECommerceShoppingCart) -> f32 {
        cart.get_total()
    }
}

pub struct DefaultDiscountStrategy;

impl DiscountStrategy for DefaultDiscountStrategy {
    fn get_discount(&self, cart: &ECommerceShoppingCart) -> f32 {
        let total = cart.get_total();
        let mut discount = 0_f32;

        if total >= 100_f32 && total <= 200_f32 {
            discount = 10_f32;
        } else if total >= 200_f32 && total <= 300_f32 {
            discount = 20_f32;
        } else if total >= 300_f32 {
            discount = 30_f32;
        }

        total * (1_f32 - discount / 100_f32)
    }
}

pub struct NewDiscountStrategy;

impl DiscountStrategy for NewDiscountStrategy {
    fn get_discount(&self, cart: &ECommerceShoppingCart) -> f32 {
        cart.get_total() * 0.95
    }
}

pub struct ECommerceProduct {
    name: String,
    price: f32,
}

impl ECommerceProduct {
    pub fn new(name: &str, price: f32) -> Self {
        let name = name.to_string();
        Self { name, price }
    }

    /// Get a reference to the ecommerce product's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the ecommerce product's price.
    pub fn price(&self) -> f32 {
        self.price
    }
}
