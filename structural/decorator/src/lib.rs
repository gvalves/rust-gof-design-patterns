use std::rc::Rc;

pub trait ProductProtocol {
    fn get_name(&self) -> String;
    fn get_price(&self) -> f64;
}

pub struct ProductStampDecorator<T: ProductProtocol>
where
    Self: ProductProtocol,
{
    product: Rc<T>,
}

impl<T: ProductProtocol> ProductStampDecorator<T>
where
    Self: ProductProtocol,
{
    pub fn new(product: Rc<T>) -> ProductStampDecorator<T> {
        ProductStampDecorator { product }
    }
}

impl<T: ProductProtocol> ProductProtocol for ProductStampDecorator<T> {
    fn get_name(&self) -> String {
        let mut name = self.product.get_name();
        name.push_str(" (Estampada)");
        name
    }

    fn get_price(&self) -> f64 {
        self.product.get_price() + 10f64
    }
}

pub struct ProductCustomizationDecorator<T: ProductProtocol>
where
    Self: ProductProtocol,
{
    product: Rc<T>,
}

impl<T: ProductProtocol> ProductCustomizationDecorator<T>
where
    Self: ProductProtocol,
{
    pub fn new(product: Rc<T>) -> ProductCustomizationDecorator<T> {
        ProductCustomizationDecorator { product }
    }
}

impl<T: ProductProtocol> ProductProtocol for ProductCustomizationDecorator<T> {
    fn get_name(&self) -> String {
        let mut name = self.product.get_name();
        name.push_str(" (Customizada)");
        name
    }

    fn get_price(&self) -> f64 {
        self.product.get_price() + 50f64
    }
}

pub struct TShirt {
    name: String,
    price: f64,
}

impl TShirt {
    pub fn new() -> TShirt {
        TShirt {
            name: "Camiseta".to_string(),
            price: 49.9,
        }
    }
}

impl ProductProtocol for TShirt {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_price(&self) -> f64 {
        self.price
    }
}
