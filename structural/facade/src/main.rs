use facade::BuilderFacade;

pub fn main() {
    let builder_facade = BuilderFacade::new();
    builder_facade.make_main_dish_beverage();

    let builder_facade = BuilderFacade::new();
    builder_facade.make_main_dish_meal_with_dessert();

    let builder_facade = BuilderFacade::new();
    builder_facade.make_vegan_dish_meal();
}
