use builder::{MainDishBuilder, VeganDishBuilder};

mod builder;

pub struct BuilderFacade {
    main_dish_builder: MainDishBuilder,
    vegan_dish_builder: VeganDishBuilder,
}

impl BuilderFacade {
    pub fn new() -> BuilderFacade {
        BuilderFacade {
            main_dish_builder: MainDishBuilder::new(),
            vegan_dish_builder: VeganDishBuilder::new(),
        }
    }

    pub fn make_main_dish_meal_with_dessert(self) {
        let Self {
            mut main_dish_builder,
            ..
        } = self;

        main_dish_builder.make_dessert();

        let meal = main_dish_builder.make_meal();

        println!("{:#?}", meal);
        println!("{}", meal.get_price());
    }

    pub fn make_main_dish_beverage(self) {
        let Self {
            mut main_dish_builder,
            ..
        } = self;

        main_dish_builder.make_beverage();

        let meal = main_dish_builder.get_meal();

        println!("{:#?}", meal);
        println!("{}", meal.get_price());
    }

    pub fn make_vegan_dish_meal(self) {
        let Self {
            vegan_dish_builder, ..
        } = self;

        let meal = vegan_dish_builder.make_meal();

        println!("{:#?}", meal);
        println!("{}", meal.get_price());
    }
}
