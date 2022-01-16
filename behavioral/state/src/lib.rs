pub trait ShoppingOrderState {
    fn name(&self) -> &str;

    fn approve_payment(self: Box<Self>) -> Box<dyn ShoppingOrderState>;
    fn reject_payment(self: Box<Self>) -> Box<dyn ShoppingOrderState>;
    fn wait_payment(self: Box<Self>) -> Box<dyn ShoppingOrderState>;

    fn ship_order(&self);
}

pub struct ShoppingOrder {
    state: Option<Box<dyn ShoppingOrderState>>,
}

impl ShoppingOrder {
    pub fn new() -> Self {
        let state = Box::new(OrderPending::new()) as Box<dyn ShoppingOrderState>;
        let state = Some(state);
        Self { state }
    }

    pub fn approve_payment(&mut self) {
        let state = self.state.take().unwrap();
        self.set_state(state.approve_payment());
        println!("O estado atual do pedido é {}", self.get_state_name());
    }

    pub fn reject_payment(&mut self) {
        let state = self.state.take().unwrap();
        self.set_state(state.reject_payment());
        println!("O estado atual do pedido é {}", self.get_state_name());
    }

    pub fn wait_payment(&mut self) {
        let state = self.state.take().unwrap();
        self.set_state(state.wait_payment());
        println!("O estado atual do pedido é {}", self.get_state_name());
    }

    pub fn ship_order(&self) {
        self.state.as_ref().unwrap().ship_order();
    }

    /// Get a reference to the shopping order's state.
    pub fn state(&self) -> &Box<dyn ShoppingOrderState> {
        self.state.as_ref().unwrap()
    }

    /// Set the shopping order's state.
    pub fn set_state(&mut self, state: Box<dyn ShoppingOrderState>) {
        self.state = Some(state);
    }

    pub fn get_state_name(&self) -> &str {
        self.state.as_ref().unwrap().name()
    }
}

pub struct OrderPending {
    name: String,
}

impl OrderPending {
    pub fn new() -> Self {
        Self {
            name: "Pending".to_string(),
        }
    }
}

impl ShoppingOrderState for OrderPending {
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn approve_payment(self: Box<Self>) -> Box<dyn ShoppingOrderState> {
        Box::new(OrderApproved::new())
    }

    fn reject_payment(self: Box<Self>) -> Box<dyn ShoppingOrderState> {
        Box::new(OrderRejected::new())
    }

    fn wait_payment(self: Box<Self>) -> Box<dyn ShoppingOrderState> {
        println!("O pedido já está pendente!");
        self
    }

    fn ship_order(&self) {
        println!("Aguardando pagamento do pedido para poder enviá-lo!");
    }
}

pub struct OrderApproved {
    name: String,
}

impl OrderApproved {
    pub fn new() -> Self {
        Self {
            name: "Approved".to_string(),
        }
    }
}

impl ShoppingOrderState for OrderApproved {
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn approve_payment(self: Box<Self>) -> Box<dyn ShoppingOrderState> {
        println!("O pedido já está aprovado!");
        self
    }

    fn reject_payment(self: Box<Self>) -> Box<dyn ShoppingOrderState> {
        Box::new(OrderRejected::new())
    }

    fn wait_payment(self: Box<Self>) -> Box<dyn ShoppingOrderState> {
        Box::new(OrderPending::new())
    }

    fn ship_order(&self) {
        println!("O pedido foi enviado!");
    }
}

pub struct OrderRejected {
    name: String,
}

impl OrderRejected {
    pub fn new() -> Self {
        Self {
            name: "Rejected".to_string(),
        }
    }
}

impl ShoppingOrderState for OrderRejected {
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn approve_payment(self: Box<Self>) -> Box<dyn ShoppingOrderState> {
        println!("O pedido foi rejeitado por isso não pode ser aprovado");
        self
    }

    fn reject_payment(self: Box<Self>) -> Box<dyn ShoppingOrderState> {
        println!("O pedido já foi rejeitado");
        self
    }

    fn wait_payment(self: Box<Self>) -> Box<dyn ShoppingOrderState> {
        Box::new(OrderPending::new())
    }

    fn ship_order(&self) {
        println!("Pedido rejeitado não pode ser enviado!");
    }
}
