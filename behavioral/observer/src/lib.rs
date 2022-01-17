use std::{any::Any, cell::RefCell, rc::Rc};

pub mod utils;

pub trait Observable {
    fn subscribe(&mut self, observer: &Rc<RefCell<dyn Observer>>);
    fn unsubscribe(&mut self, observer: &Rc<RefCell<dyn Observer>>);
    fn notify(&self);
}

pub trait Observer {
    fn update(&mut self, data: Option<Box<dyn Any>>);
}

pub struct InputObservable {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    element: HTMLElement,
    events: Vec<Box<dyn Fn(InputObservableState<'_>)>>,
}

impl InputObservable {
    pub fn new() -> Self {
        Self {
            observers: vec![],
            element: HTMLElement::new("Input"),
            events: vec![],
        }
    }

    pub fn add_event(&mut self, event: Box<dyn Fn(InputObservableState<'_>)>) {
        self.events.push(event);
    }

    /// Get a reference to the input observable's observers.
    pub fn observers(&self) -> &[Rc<RefCell<dyn Observer>>] {
        self.observers.as_ref()
    }

    /// Get a reference to the input observable's element.
    pub fn element(&self) -> &HTMLElement {
        &self.element
    }

    pub fn set_value(&mut self, value: String) {
        self.element.set_value(value);
        self.events
            .iter()
            .for_each(|event| event(InputObservableState::new(&self)));
    }
}

impl Observable for InputObservable {
    fn subscribe(&mut self, observer: &Rc<RefCell<dyn Observer>>) {
        let observer = Rc::clone(observer);
        self.observers.push(observer);
    }

    fn unsubscribe(&mut self, observer: &Rc<RefCell<dyn Observer>>) {
        let position = self
            .observers
            .iter()
            .position(|_observer| Rc::ptr_eq(observer, _observer));

        let index = match position {
            Some(index) => index,
            None => return,
        };

        self.observers.remove(index);
    }

    fn notify(&self) {
        self.observers.iter().for_each(|observer| {
            let value = self.element.value().to_string();
            let data = Some(Box::new(value) as Box<dyn Any>);
            observer.borrow_mut().update(data);
        });
    }
}

pub struct InputObservableState<'a> {
    input_observable: &'a InputObservable,
}

impl<'a> InputObservableState<'a> {
    pub fn new(input_observable: &'a InputObservable) -> Self {
        Self { input_observable }
    }

    /// Get a reference to the input observable state's input observable.
    pub fn input_observable(&self) -> &InputObservable {
        self.input_observable
    }
}

pub struct ParagraphObserver {
    element: HTMLElement,
}

impl ParagraphObserver {
    pub fn new() -> Self {
        Self {
            element: HTMLElement::new("Paragraph"),
        }
    }

    /// Get a reference to the paragraph observer's element.
    pub fn element(&self) -> &HTMLElement {
        &self.element
    }
}

impl Observer for ParagraphObserver {
    fn update(&mut self, data: Option<Box<dyn Any>>) {
        let data = data.unwrap();
        let input_value = match data.downcast::<String>() {
            Ok(val) => val,
            Err(_) => return,
        };
        self.element.set_value(*input_value);
    }
}

impl From<ParagraphObserver> for Rc<RefCell<dyn Observer>> {
    fn from(value: ParagraphObserver) -> Self {
        Rc::new(RefCell::new(value))
    }
}

pub struct DivObserver {
    element: HTMLElement,
}

impl DivObserver {
    pub fn new() -> Self {
        Self {
            element: HTMLElement::new("Div"),
        }
    }

    /// Get a reference to the paragraph observer's element.
    pub fn element(&self) -> &HTMLElement {
        &self.element
    }
}

impl Observer for DivObserver {
    fn update(&mut self, data: Option<Box<dyn Any>>) {
        let data = data.unwrap();
        let input_value = match data.downcast::<String>() {
            Ok(val) => val,
            Err(_) => return,
        };
        self.element.set_value(*input_value);
    }
}

impl From<DivObserver> for Rc<RefCell<dyn Observer>> {
    fn from(value: DivObserver) -> Self {
        Rc::new(RefCell::new(value))
    }
}

pub struct HTMLElement {
    typ: String,
    value: String,
}

impl HTMLElement {
    pub fn new(typ: &str) -> Self {
        let typ = typ.to_string();
        Self {
            typ,
            value: String::new(),
        }
    }

    /// Get a reference to the htmlinput element's value.
    pub fn value(&self) -> &str {
        self.value.as_ref()
    }

    /// Set the htmlinput element's value.
    pub fn set_value(&mut self, value: String) {
        println!(
            "HTML{}Element ( old_val: {}, new_val: {} )",
            self.typ, self.value, value
        );
        self.value = value;
    }

    /// Get a reference to the htmlelement's typ.
    pub fn typ(&self) -> &str {
        self.typ.as_ref()
    }
}
