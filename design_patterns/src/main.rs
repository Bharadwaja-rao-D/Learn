use design_patterns::decorator::{Component, Decorations, MenuBar, ScrollBar};

fn main() {
    let with_scrollbar = ScrollBar::new(vec![Decorations::Nothing]);
    with_scrollbar.draw();
    println!("--------------------");

    let with_scrollbar_menubar = MenuBar::new(vec![Decorations::Nothing, Decorations::ScrollBar]);
    with_scrollbar_menubar.draw();
    println!("--------------------");

    let test = MenuBar::new(vec![
        Decorations::Nothing,
        Decorations::ScrollBar,
        Decorations::MenuBar,
        Decorations::ScrollBar,
        Decorations::MenuBar,
    ]);
    test.draw();
}
