pub trait ProductComponent
where
    Self: std::fmt::Debug,
    Self: Sync,
{
    fn get_price(&self) -> usize;

    fn get_name(&self) -> &str {
        ""
    }

    fn get_children(&self) -> &Vec<Box<dyn ProductComponent>> {
        static CHILDREN: &Vec<Box<dyn ProductComponent>> = &vec![];
        CHILDREN
    }

    #[allow(unused_variables)]
    fn add(&mut self, products: &mut Vec<Box<dyn ProductComponent>>) {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn remove(&mut self, product: Box<dyn ProductComponent>) {
        unimplemented!()
    }
}

impl PartialEq for Box<dyn ProductComponent> {
    fn eq(&self, other: &Self) -> bool {
        self.get_price() == other.get_price()
            && self.get_name() == other.get_name()
            && self.get_children() == other.get_children()
    }
}

#[derive(Debug, Clone)]
pub struct ProductLeaf {
    name: String,
    price: usize,
}

impl ProductLeaf {
    pub fn new(name: &str, price: usize) -> ProductLeaf {
        let name = name.into();
        ProductLeaf { name, price }
    }
}

impl ProductComponent for ProductLeaf {
    fn get_price(&self) -> usize {
        self.price
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

impl From<ProductLeaf> for Box<dyn ProductComponent> {
    fn from(leaf: ProductLeaf) -> Self {
        Box::new(leaf)
    }
}

#[derive(Debug)]
pub struct ProductComposite {
    children: Vec<Box<dyn ProductComponent>>,
}

impl ProductComposite {
    pub fn new() -> ProductComposite {
        ProductComposite { children: vec![] }
    }
}

impl ProductComponent for ProductComposite {
    fn get_price(&self) -> usize {
        self.children
            .iter()
            .fold(0, |sum, product| sum + product.get_price())
    }

    fn get_children(&self) -> &Vec<Box<dyn ProductComponent>> {
        &self.children
    }

    fn add(&mut self, products: &mut Vec<Box<dyn ProductComponent>>) {
        self.children.append(products);
    }

    fn remove(&mut self, product: Box<dyn ProductComponent>) {
        let position = self.children.iter().position(|child| product.eq(&child));
        let index = match position {
            Some(index) => index,
            None => return,
        };

        self.children.remove(index);
    }
}

impl From<ProductComposite> for Box<dyn ProductComponent> {
    fn from(composite: ProductComposite) -> Self {
        Box::new(composite)
    }
}
