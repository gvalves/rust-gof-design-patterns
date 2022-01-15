use flyweight::{delivery_context, DeliveryFactory};

pub fn main() {
    let mut factory = DeliveryFactory::new();

    delivery_context(&mut factory, "Gustavo", "30", "Av Brasil", "São Paulo");
    delivery_context(&mut factory, "Bruno", "30", "Av Brasil", "São Paulo");
    delivery_context(&mut factory, "Marcelo", "80", "Av Brasil", "São Paulo");
    delivery_context(&mut factory, "Maria", "80", "Rua A", "São Paulo");
    delivery_context(&mut factory, "João", "803", "Rua B", "São Paulo");

    println!();
    println!("{:#?}", factory);
}
