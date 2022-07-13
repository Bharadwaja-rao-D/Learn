pub enum Decorations {
    Nothing,
    ScrollBar,
    MenuBar,
}

pub trait Component {
    fn draw(&self);
}

pub struct MainComponent {
    pub name: String,
}

impl Component for MainComponent {
    fn draw(&self) {
        println!("MainComponent: {}", self.name);
    }
}

impl MainComponent {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
        };
    }
}

pub trait DecoratorComponent {}

pub struct ScrollBar {
    // Aggregation
    pub obj: Box<dyn Component>,
    pub r#type: String,
}

impl DecoratorComponent for ScrollBar {}

impl ScrollBar {
    pub fn another_new(obj: Box<dyn Component>) -> Self {
        return Self {
            obj,
            r#type: "ScrollBar".to_string(),
        };
    }
    pub fn new(decorations: Vec<Decorations>) -> Self {
        return Self {
            obj: build_top(decorations),
            r#type: "ScrollBar".to_string(),
        };
    }
}

impl Component for ScrollBar {
    fn draw(&self) {
        self.obj.draw();
        println!("{}", self.r#type);
    }
}

pub struct MenuBar {
    // Aggregation
    pub obj: Box<dyn Component>,
    pub r#type: String,
}

impl MenuBar {
    pub fn another_new(obj: Box<dyn Component>) -> Self {
        return Self {
            obj,
            r#type: "ScrollBar".to_string(),
        };
    }
    pub fn new(decorations: Vec<Decorations>) -> Self {
        return Self {
            obj: build_top(decorations),
            r#type: "MenuBar".to_string(),
        };
    }
}

impl Component for MenuBar {
    fn draw(&self) {
        self.obj.draw();
        println!("{}", self.r#type);
    }
}

pub fn build_top(decorations: Vec<Decorations>) -> Box<dyn Component> {
    let mut obj: Box<dyn Component> = Box::new(MainComponent::new("TEST"));
    for deco in decorations.iter() {
        match deco {
            Decorations::ScrollBar => {
                obj = Box::new(ScrollBar::another_new(obj));
            }
            Decorations::MenuBar => {
                obj = Box::new(MenuBar::another_new(obj));
            }
            _ => {}
        }
    }

    return obj;
}
