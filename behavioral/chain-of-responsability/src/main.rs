use chain_of_responsability::{
    BudgetHandler, CeoBudgetHandler, CustomerBudget, DirectorBudgetHandler, ManagerBudgetHandler,
    SellerBudgetHandler,
};

pub fn main() {
    let customer_budget = CustomerBudget::new(9_999_999_f64);

    let mut seller = SellerBudgetHandler::new();

    seller
        .set_next_handler(Box::new(ManagerBudgetHandler::new()))
        .set_next_handler(Box::new(DirectorBudgetHandler::new()))
        .set_next_handler(Box::new(CeoBudgetHandler::new()));

    seller.handle(customer_budget);
}
