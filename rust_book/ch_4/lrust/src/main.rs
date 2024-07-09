fn main() {
    let mut name = read_for_cat();
    name = hay_name(&name);

    println!("{name}");

    let str = second_word(&name);

    println!("{str}");
}
fn read_for_cat() -> String {
    let mut string = String::new();

    std::io::stdin()
        .read_line(&mut string)
        .expect("Error with read");

    string.trim().to_string()
}
fn hay_name(name: &str) -> String {
    let mut hello: String = String::from("Hello");

    {
        let hay: String = String::from("how are you?");
        let point = String::from(", ");

        hello.push_str(&point);
        hello.push_str(&name);
        hello.push_str(&point);
        hello.push_str(&hay);
    }
    hello
}
fn links() {
    let mut var = String::from("hello");

    let l1 = &var;
    let l2 = &var;

    println!("{l1} {l2}");

    let l3 = &mut var;

    println!("{l3}");
}
fn ownership_func(var: String) -> String {
    let str: String = String::from("hello");

    str
}
fn first_word(str: &str) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }

    &str[..]
}
fn second_word(str: &str) -> &str {
    let bytes = str.as_bytes();

    let i = 0;
    let mut ret_val: &str;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let j = i;

            for (j, &element) in bytes.iter().enumerate() {
                if element == b' ' {
                    println!("{}", &str[i..j]);
                }
            }
            return &str[..i];
        }
    }

    &str[..i]
}
