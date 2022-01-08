use factory_method::utils::{random_number, random_vehicle};

fn main() {
    let customer_names = ["Ana", "Joana", "Helena", "João"];

    {
        let mut i = 0;

        while i < 10 {
            let vehicle = random_vehicle();
            let max = customer_names.len();
            let customer_name = customer_names[random_number(max)];

            vehicle.pick_up(customer_name);
            vehicle.stop();

            println!("{}", "-".repeat(10));

            i += 1;
        }
    }
}
