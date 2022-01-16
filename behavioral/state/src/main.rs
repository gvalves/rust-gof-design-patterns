use state::ShoppingOrder;

pub fn main() {
    let mut order = ShoppingOrder::new();

    order.approve_payment();
    order.wait_payment();
    order.reject_payment();

    order.ship_order();
}
