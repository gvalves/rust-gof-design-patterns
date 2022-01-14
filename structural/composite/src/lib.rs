use std::rc::Rc;

use product::{ProductComponent, ProductComposite, ProductLeaf};

use crate::validation::{
    ValidationComponent, ValidationComposite, ValidationEmail, ValidationNumber, ValidationString,
};

pub mod product;
pub mod validation;

pub fn run_product() {
    let pen = ProductLeaf::new("Caneta", 1);
    let smartphone = ProductLeaf::new("Smartphone", 1_000);
    let t_shirt = ProductLeaf::new("Camiseta", 40);

    let mut products = vec![pen.into(), smartphone.clone().into(), t_shirt.into()];

    let mut product_box = ProductComposite::new();
    product_box.add(&mut products);

    println!("{:#?}", product_box);
    println!("{}", product_box.get_price());

    let tablet = ProductLeaf::new("Tablet", 2_000);
    let kindle = ProductLeaf::new("Kindle", 300);

    let mut another_product_box = ProductComposite::new();
    another_product_box.add(&mut vec![tablet.into(), kindle.into()]);
    product_box.add(&mut vec![another_product_box.into()]);

    println!("{:#?}", product_box);
    product_box.remove(smartphone.into());
    println!("{}", product_box.get_price());
}

pub fn run_validation() {
    let validation_email = ValidationEmail::new();
    let validation_number = ValidationNumber::new();
    let validation_string = ValidationString::new();

    let mut validation_composite = ValidationComposite::new();
    validation_composite.add(&mut vec![
        validation_email.into(),
        validation_number.into(),
        validation_string.into(),
    ]);

    println!(
        "{}",
        validation_composite.validate(Rc::new("luiz123@gmail.com".to_string()))
    );
}
