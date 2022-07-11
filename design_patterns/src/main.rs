use design_patterns::composite::{Component, Composite, Leaf};

fn main() {
    let mut manager1 = Composite::new("Manager1");
    let mut manager2 = Composite::new("Manager2");
    let employee1 = Leaf::new("Employee1");
    let employee2 = Leaf::new("Employee2");

    //TODO: The problem is here the object is getting consumed
    manager2.add_child(Box::new(employee2));
    manager2.add_child(Box::new(employee1));

    manager1.add_child(Box::new(manager2));

    manager1.draw();

    println!("--------------");
}
