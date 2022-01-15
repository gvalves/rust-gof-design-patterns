use futures::{self, executor::block_on};
use proxy::SystemUserProxy;

pub async fn async_main() {
    let mut user = SystemUserProxy::new("Gustavo", "gvalves");
    println!("Isso vai levar 2 segundos");
    println!("{:#?}", user.get_addresses().await);

    println!("\nLOOP\n");

    for _ in 1..5 {
        println!("{:#?}", user.get_addresses().await);
    }
}

pub fn main() {
    block_on(async_main());
}
