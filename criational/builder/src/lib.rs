pub trait Meal {
    fn get_price(&self) -> usize;
}

pub trait MealBuilder {
    fn make_beans(&mut self, price: usize) -> &mut Self;
    fn make_beverage(&mut self, price: usize) -> &mut Self;
    fn make_dessert(&mut self, price: usize) -> &mut Self;
    fn make_meat(&mut self, price: usize) -> &mut Self;
    fn make_rice(&mut self, price: usize) -> &mut Self;
    fn make_meal(self) -> MealBox;
}

pub type TMeal = Box<dyn Meal>;

pub struct MealBox {
    meals: Vec<TMeal>,
}

impl MealBox {
    pub fn new() -> MealBox {
        MealBox { meals: vec![] }
    }

    pub fn add(&mut self, meal: TMeal) {
        self.meals.push(meal);
    }

    pub fn get_price(&self) -> usize {
        Meal::get_price(self)
    }
}

impl Meal for MealBox {
    fn get_price(&self) -> usize {
        self.meals
            .iter()
            .fold(0, |acc, meal| acc + meal.get_price())
    }
}

pub struct Beans {
    price: usize,
}

impl Beans {
    pub fn new(price: usize) -> Beans {
        Beans { price }
    }
}

impl Meal for Beans {
    fn get_price(&self) -> usize {
        self.price
    }
}

pub struct Beverage {
    price: usize,
}

impl Beverage {
    pub fn new(price: usize) -> Beverage {
        Beverage { price }
    }
}

impl Meal for Beverage {
    fn get_price(&self) -> usize {
        self.price
    }
}

pub struct Dessert {
    price: usize,
}

impl Dessert {
    pub fn new(price: usize) -> Dessert {
        Dessert { price }
    }
}

impl Meal for Dessert {
    fn get_price(&self) -> usize {
        self.price
    }
}

pub struct Meat {
    price: usize,
}

impl Meat {
    pub fn new(price: usize) -> Meat {
        Meat { price }
    }
}

impl Meal for Meat {
    fn get_price(&self) -> usize {
        self.price
    }
}

pub struct Rice {
    price: usize,
}

impl Rice {
    pub fn new(price: usize) -> Rice {
        Rice { price }
    }
}

impl Meal for Rice {
    fn get_price(&self) -> usize {
        self.price
    }
}

pub struct DishBuilder {
    meal: MealBox,
    beans: Option<Beans>,
    beverage: Option<Beverage>,
    dessert: Option<Dessert>,
    meat: Option<Meat>,
    rice: Option<Rice>,
}

impl DishBuilder {
    pub fn new() -> DishBuilder {
        DishBuilder {
            meal: MealBox::new(),
            beans: None,
            beverage: None,
            dessert: None,
            meat: None,
            rice: None,
        }
    }
}

impl MealBuilder for DishBuilder {
    fn make_beans(&mut self, price: usize) -> &mut Self {
        self.beans = Some(Beans::new(price));
        self
    }

    fn make_beverage(&mut self, price: usize) -> &mut Self {
        self.beverage = Some(Beverage::new(price));
        self
    }

    fn make_dessert(&mut self, price: usize) -> &mut Self {
        self.dessert = Some(Dessert::new(price));
        self
    }

    fn make_meat(&mut self, price: usize) -> &mut Self {
        self.meat = Some(Meat::new(price));
        self
    }

    fn make_rice(&mut self, price: usize) -> &mut Self {
        self.rice = Some(Rice::new(price));
        self
    }

    fn make_meal(self) -> MealBox {
        let mut meal = self.meal;

        if let Some(beans) = self.beans {
            meal.add(Box::new(beans));
        }

        if let Some(beverage) = self.beverage {
            meal.add(Box::new(beverage));
        }

        if let Some(dessert) = self.dessert {
            meal.add(Box::new(dessert));
        }

        if let Some(meat) = self.meat {
            meal.add(Box::new(meat));
        }

        if let Some(rice) = self.rice {
            meal.add(Box::new(rice));
        }

        meal
    }
}

pub struct MainDishBuilder {
    dish_builder: DishBuilder,
}

impl MainDishBuilder {
    pub fn new() -> MainDishBuilder {
        MainDishBuilder {
            dish_builder: DishBuilder::new(),
        }
    }

    pub fn make_dessert(&mut self) -> &mut Self {
        self.dish_builder.make_dessert(10);
        self
    }

    pub fn make_beverage(&mut self) -> &mut Self {
        self.dish_builder.make_beverage(7);
        self
    }

    pub fn make_meal(self) -> MealBox {
        let Self { mut dish_builder } = self;

        dish_builder.make_rice(5);
        dish_builder.make_beans(10);
        dish_builder.make_meat(20);

        dish_builder.make_meal()
    }
}

pub struct VeganDishBuilder {
    dish_builder: DishBuilder,
}

impl VeganDishBuilder {
    pub fn new() -> VeganDishBuilder {
        VeganDishBuilder {
            dish_builder: DishBuilder::new(),
        }
    }

    pub fn make_meal(self) -> MealBox {
        let Self { mut dish_builder } = self;

        dish_builder.make_rice(5);
        dish_builder.make_beans(10);

        dish_builder.make_meal()
    }
}
