pub trait VisitableProduct {
    fn name(&self) -> &str;
    fn price(&self) -> f32;
    fn get_price_with_taxes(&self, visitor: &Box<dyn TaxVisitor>) -> f32;
}

pub trait TaxVisitor {
    fn calc_taxes_for_food(&self, food: &Food) -> f32;
    fn calc_taxes_for_cigarette(&self, cigarette: &Cigarette) -> f32;
    fn calc_taxes_for_alcoholic_drink(&self, alcoholic_drink: &AlcoholicDrink) -> f32;
}

pub struct Food {
    name: String,
    price: f32,
}

impl Food {
    pub fn new(price: f32) -> Self {
        let name = "Food".to_string();
        Self { name, price }
    }
}

impl VisitableProduct for Food {
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn price(&self) -> f32 {
        self.price
    }

    fn get_price_with_taxes(&self, visitor: &Box<dyn TaxVisitor>) -> f32 {
        visitor.calc_taxes_for_food(self)
    }
}

pub struct Cigarette {
    name: String,
    price: f32,
}

impl Cigarette {
    pub fn new(price: f32) -> Self {
        let name = "Cigarette".to_string();
        Self { name, price }
    }
}

impl VisitableProduct for Cigarette {
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn price(&self) -> f32 {
        self.price
    }

    fn get_price_with_taxes(&self, visitor: &Box<dyn TaxVisitor>) -> f32 {
        visitor.calc_taxes_for_cigarette(self)
    }
}
pub struct AlcoholicDrink {
    name: String,
    price: f32,
}

impl AlcoholicDrink {
    pub fn new(price: f32) -> Self {
        let name = "AlcoholicDrink".to_string();
        Self { name, price }
    }
}

impl VisitableProduct for AlcoholicDrink {
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn price(&self) -> f32 {
        self.price
    }

    fn get_price_with_taxes(&self, visitor: &Box<dyn TaxVisitor>) -> f32 {
        visitor.calc_taxes_for_alcoholic_drink(self)
    }
}

pub struct BrasilTaxVisitor;

impl TaxVisitor for BrasilTaxVisitor {
    fn calc_taxes_for_food(&self, food: &Food) -> f32 {
        food.price() * 1.05
    }

    fn calc_taxes_for_cigarette(&self, cigarette: &Cigarette) -> f32 {
        cigarette.price() * 2.5
    }

    fn calc_taxes_for_alcoholic_drink(&self, alcoholic_drink: &AlcoholicDrink) -> f32 {
        alcoholic_drink.price() * 1.5
    }
}

pub struct UsTaxVisitor;

impl TaxVisitor for UsTaxVisitor {
    fn calc_taxes_for_food(&self, food: &Food) -> f32 {
        food.price() * 1.15
    }

    fn calc_taxes_for_cigarette(&self, cigarette: &Cigarette) -> f32 {
        cigarette.price() * 3.0
    }

    fn calc_taxes_for_alcoholic_drink(&self, alcoholic_drink: &AlcoholicDrink) -> f32 {
        alcoholic_drink.price() * 2.0
    }
}
