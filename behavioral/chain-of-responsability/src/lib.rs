pub trait BudgetHandler {
    fn handle(&self, budget: CustomerBudget) -> CustomerBudget;
    fn set_next_handler(&mut self, handler: Box<dyn BudgetHandler>) -> &mut Box<dyn BudgetHandler>;
}

pub struct CustomerBudget {
    total: f64,
    approved: bool,
}

impl CustomerBudget {
    pub fn new(total: f64) -> Self {
        Self {
            total,
            approved: false,
        }
    }

    pub fn approve(&mut self) {
        self.approved = true;
    }

    /// Get a reference to the customer budget's total.
    pub fn total(&self) -> f64 {
        self.total
    }

    /// Get a reference to the customer budget's approved.
    pub fn is_approved(&self) -> bool {
        self.approved
    }
}

pub struct BaseBudgetHandler {
    next_handler: Option<Box<dyn BudgetHandler>>,
}

impl BaseBudgetHandler {
    pub fn new() -> Self {
        Self { next_handler: None }
    }
}

impl BudgetHandler for BaseBudgetHandler {
    fn handle(&self, budget: CustomerBudget) -> CustomerBudget {
        match self.next_handler.as_ref() {
            Some(handler) => handler.handle(budget),
            None => budget,
        }
    }

    fn set_next_handler(&mut self, handler: Box<dyn BudgetHandler>) -> &mut Box<dyn BudgetHandler> {
        self.next_handler = Some(handler);
        self.next_handler.as_mut().unwrap()
    }
}

pub struct SellerBudgetHandler {
    base_budget_handler: BaseBudgetHandler,
}

impl SellerBudgetHandler {
    pub fn new() -> Self {
        Self {
            base_budget_handler: BaseBudgetHandler::new(),
        }
    }
}

impl BudgetHandler for SellerBudgetHandler {
    fn handle(&self, budget: CustomerBudget) -> CustomerBudget {
        let mut budget = budget;

        if budget.total() <= 1000f64 {
            println!("O vendedor tratou o orçamento!");
            budget.approve();
            return budget;
        }

        self.base_budget_handler.handle(budget)
    }

    fn set_next_handler(&mut self, handler: Box<dyn BudgetHandler>) -> &mut Box<dyn BudgetHandler> {
        self.base_budget_handler.set_next_handler(handler)
    }
}

pub struct ManagerBudgetHandler {
    base_budget_handler: BaseBudgetHandler,
}

impl ManagerBudgetHandler {
    pub fn new() -> Self {
        Self {
            base_budget_handler: BaseBudgetHandler::new(),
        }
    }
}

impl BudgetHandler for ManagerBudgetHandler {
    fn handle(&self, budget: CustomerBudget) -> CustomerBudget {
        let mut budget = budget;

        if budget.total() <= 5000f64 {
            println!("O gerente tratou o orçamento!");
            budget.approve();
            return budget;
        }

        self.base_budget_handler.handle(budget)
    }

    fn set_next_handler(&mut self, handler: Box<dyn BudgetHandler>) -> &mut Box<dyn BudgetHandler> {
        self.base_budget_handler.set_next_handler(handler)
    }
}

pub struct DirectorBudgetHandler {
    base_budget_handler: BaseBudgetHandler,
}

impl DirectorBudgetHandler {
    pub fn new() -> Self {
        Self {
            base_budget_handler: BaseBudgetHandler::new(),
        }
    }
}

impl BudgetHandler for DirectorBudgetHandler {
    fn handle(&self, budget: CustomerBudget) -> CustomerBudget {
        let mut budget = budget;

        if budget.total() <= 50000f64 {
            println!("O diretor tratou o orçamento!");
            budget.approve();
            return budget;
        }

        self.base_budget_handler.handle(budget)
    }

    fn set_next_handler(&mut self, handler: Box<dyn BudgetHandler>) -> &mut Box<dyn BudgetHandler> {
        self.base_budget_handler.set_next_handler(handler)
    }
}

pub struct CeoBudgetHandler {
    base_budget_handler: BaseBudgetHandler,
}

impl CeoBudgetHandler {
    pub fn new() -> Self {
        Self {
            base_budget_handler: BaseBudgetHandler::new(),
        }
    }
}

impl BudgetHandler for CeoBudgetHandler {
    fn handle(&self, budget: CustomerBudget) -> CustomerBudget {
        let mut budget = budget;

        println!("O CEO trata qualquer  orçamento!");
        budget.approve();

        return budget;
    }

    fn set_next_handler(&mut self, handler: Box<dyn BudgetHandler>) -> &mut Box<dyn BudgetHandler> {
        self.base_budget_handler.set_next_handler(handler)
    }
}
