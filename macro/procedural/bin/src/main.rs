use lib::{log_function, var_name, Descripe};

use use_macro::{use_attribute_macro, use_descripe, use_var_name};
mod use_macro {
    use super::*;

    pub fn use_var_name() {
        let s = String::from("hello");
        let hello = String::from("s");

        var_name!(s);
        var_name!(hello);
    }
    pub fn use_descripe() {
        trait Description {
            fn descripe() -> String;
        }

        #[derive(Descripe)]
        struct Pipiska;

        println!("{}", Pipiska::descripe());
    }

    pub fn use_attribute_macro() {
        #[log_function]
        fn meow() {
            println!("Meow");
        }

        meow();
    }
}

fn main() {
    use_var_name();
    use_descripe();
    use_attribute_macro();
}
