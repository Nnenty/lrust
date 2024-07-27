use use_macros::{use_power, use_vecc};
mod use_macros {
    use super::*;

    pub fn use_vecc() {
        let v = vecc!(1, 2, 3);

        assert_eq!(v, [1, 2, 3]);
    }
    pub fn use_power() {
        let sq = power!(2i32, squered);
        let cb = power!(2i32, cubed);

        assert_eq!(sq, 4);
        assert_eq!(cb, 8);
    }
}

fn main() {
    use_vecc();
    use_power();
}
#[macro_export]
macro_rules! power {
    ($value:expr, squered) => {{
        let v = $value.pow(2);
        v
    }};
    ($value:expr, cubed) => {
        $value.pow(3)
    };
}

#[macro_export]
macro_rules! vecc {
    ( $($x:expr), *) => {{
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }};
}
