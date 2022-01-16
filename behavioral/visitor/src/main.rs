use visitor::*;

pub fn main() {
    let food = Food::new(10.0);
    let cigarette = Cigarette::new(5.0);
    let alcoholic_drink = AlcoholicDrink::new(7.0);

    let brazil_tax_visitor = Box::new(BrasilTaxVisitor) as Box<dyn TaxVisitor>;
    let us_tax_visitor = Box::new(UsTaxVisitor) as Box<dyn TaxVisitor>;

    let cart: Vec<Box<dyn VisitableProduct>> = vec![
        Box::new(food),
        Box::new(cigarette),
        Box::new(alcoholic_drink),
    ];
    let total = cart.iter().fold(0.0, |sum, product| sum + product.price());
    let total_with_brasil_taxes = cart.iter().fold(0.0, |sum, product| {
        sum + product.get_price_with_taxes(&brazil_tax_visitor)
    });
    let total_with_us_taxes = cart.iter().fold(0.0, |sum, product| {
        sum + product.get_price_with_taxes(&us_tax_visitor)
    });

    println!("{}", total);
    println!("{}", total_with_brasil_taxes);
    println!("{}", total_with_us_taxes);
}
