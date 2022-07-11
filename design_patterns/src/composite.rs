pub trait Component {
    fn draw(&self);
    fn new(name: String) -> Self;
}

pub struct Head {
    pub name: String,
}

impl Component for Head {
    fn draw(&self) {
        println!("Head: {}", self.name);
    }

    fn new(name: String) -> Self {
        return Head { name };
    }
}

pub struct Composite<T: Component> {
    pub name: String,
    pub children: Vec<Box<T>>,
}

impl<T: Component> Component for Composite<T> {
    fn draw(&self) {
        println!("Composite: {}", self.name);
        for child in &self.children {
            child.draw();
        }
    }

    fn new(name: String) -> Self {
        return Composite {
            name,
            children: vec![],
        };
    }
}

impl<T: Component> Composite<T> {
    pub fn add_child(&mut self, child: T) {
        self.children.push(Box::new(child));
    }
}

pub struct Leaf {
    pub name: String,
}

impl Component for Leaf {
    fn new(name: String) -> Self {
        return Self { name };
    }
    fn draw(&self) {
        println!("Leaf: {}", self.name);
    }
}
