use builder::{MainDishBuilder, VeganDishBuilder};

fn main() {
    let mut main_dish_builder = MainDishBuilder::new();
    main_dish_builder.make_dessert();

    let main_meal = main_dish_builder.make_meal();
    println!("{}", main_meal.get_price());

    let vegan_dish_builder = VeganDishBuilder::new();
    let vegan_meal = vegan_dish_builder.make_meal();
    println!("{}", vegan_meal.get_price());
}
