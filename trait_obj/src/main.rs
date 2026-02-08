use trait_obj::{Button, Draw, Screen};


struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing the select box")
    }
}

fn main() {
    let screen = Screen{
        components: vec![
            // Box::new(String::from("Temu")), // Will fail because String does not implement the draw function
            Box::new(SelectBox{
                width: 100,
                height: 100,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ]
            }),
            Box::new(Button {
                width: 100,
                height: 100,
                label: String::from("Ok")
            })
        ]
    };

    screen.run();
}
