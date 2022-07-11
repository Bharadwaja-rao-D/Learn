pub trait Component {
    fn draw(&self);
}

pub struct Composite {
    pub name: String,
    pub children: Vec<Box<dyn Component>>,
}

impl Component for Composite {
    fn draw(&self) {
        println!("Composite: {} and the children are", self.name);
        for child in &self.children {
            child.draw();
        }
    }
}

impl Composite {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
            children: vec![],
        };
    }
    pub fn add_child(&mut self, child: Box<dyn Component>) {
        self.children.push(child);
    }
}

pub struct Leaf {
    pub name: String,
}

impl Component for Leaf {
    fn draw(&self) {
        println!("Leaf: {}", self.name);
    }
}

impl Leaf {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
        };
    }
}
