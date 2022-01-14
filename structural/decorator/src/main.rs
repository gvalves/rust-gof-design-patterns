use std::rc::Rc;

use decorator::{ProductCustomizationDecorator, ProductProtocol, ProductStampDecorator, TShirt};

pub fn main() {
    let t_shirt = TShirt::new().into();
    let t_shirt_with_one_stamp = ProductStampDecorator::new(Rc::clone(&t_shirt)).into();
    let customized_t_shirt = ProductCustomizationDecorator::new(Rc::clone(&t_shirt_with_one_stamp));

    println!("Price: {} of {}", t_shirt.get_price(), t_shirt.get_name());
    println!(
        "Price: {} of {}",
        t_shirt_with_one_stamp.get_price(),
        t_shirt_with_one_stamp.get_name()
    );
    println!(
        "Price: {} of {}",
        customized_t_shirt.get_price(),
        customized_t_shirt.get_name()
    );
}
