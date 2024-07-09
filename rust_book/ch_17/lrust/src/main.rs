use learn_encapsulation::use_ac;
mod learn_encapsulation {
    use lrust_lib::AveragedCollection;

    pub fn use_ac() {
        let mut ac = AveragedCollection::new();
        ac.add(3);
        ac.add(6);

        println!("{ac:?}");

        ac.delete();

        println!("{ac:?}");
    }
}

use learn_trait_objects::use_gui;
mod learn_trait_objects {
    use lrust_lib::{Button, Draw, Screen};

    #[derive(Debug)]
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("drawing my own type: {self:?}");
        }
    }
    impl SelectBox {
        pub fn new(width: u32, height: u32, options: Vec<String>) -> Self {
            Self {
                width: width,
                height: height,
                options: options,
            }
        }
    }

    pub fn use_gui() {
        let mut screen = Screen::new();
        let slbox = SelectBox::new(10, 20, vec!["opt1".to_string(), "opt2".to_string()]);
        let but = Button::new(15, 25, "label".to_string());

        screen.add_component(Box::new(slbox));
        screen.add_component(Box::new(but));

        screen.run();
    }
}

use learn_state::{blog_analog, blog_post};
mod learn_state {
    use lrust_lib::{Post, PostAnalog};

    pub fn blog_post() {
        let mut s = Post::new();
        s.add_text_to_end("text for post in my blog");

        println!("{}", s.content());

        s.request_review();

        println!("{}", s.content());

        s.approve();

        println!("{}", s.content());
    }
    pub fn blog_analog() {
        let mut s = PostAnalog::new();

        s.add_text("second text for post in my blog");
        let s = s.request_review();
        let s = s.approve();

        println!("content: {}", s.content());
    }
}
fn main() {
    // 1. Encapsulation (learn_encapsulation module)
    //
    // use_ac();

    // 2. Trait objects (learn_trait_objects module)
    //
    // use_gui();

    // 3. State (learn_state module)
    //
    // blog_post();
    // blog_analog();
}
