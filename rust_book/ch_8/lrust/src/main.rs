use std::{collections::HashMap, hash::Hash, string};

fn mult_users() {
    let count = 2;

    let mut user = String::new();
    let mut list: Vec<String> = Vec::new();

    for i in 0..count {
        println!("Enter your name:");

        std::io::stdin()
            .read_line(&mut user)
            .expect("error with input read");

        list.push(user.trim().to_string().clone());
        user.clear();
    }

    println!("\nUser list:");
    for (i, name) in list.iter().enumerate() {
        println!("{}. {}", i + 1, name);
    }
}
fn dyn_keys() {
    let mut hmap: HashMap<String, i32> = HashMap::new();

    for i in 1..10 {
        let istr = i.to_string();

        let mut str = String::from("command");
        str.push_str(&istr);

        hmap.insert(str, i + 10);
    }

    for i in &hmap {
        println!("{} - {} points", i.0, i.1);
    }
}
fn hmapkey_shading() {
    let mut hmap = HashMap::new();

    hmap.insert(String::from("Hello"), String::from(" world1"));
    hmap.insert(String::from("Hello"), String::from(" мир"));

    println!("{hmap:?}");
}
fn plus_num() {
    let num = 10;

    let mut vec: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for i in &mut vec {
        *i += num;

        println!("{i}");
    }
}
fn word_count() {
    let text = String::from("hello kitty cute kitty");
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);

        *count += 1;
    }

    println!("{map:?}");
}
fn different_hello() {
    let hello_j = String::from("こんにちは");
    let hello_k = String::from("안녕하세요");

    println!("As chars:");
    for i in hello_k.chars() {
        print!("{i}");
    }

    println!("\n\nAs bytes:");
    for (i, c) in hello_j.bytes().enumerate() {
        if i % 3 == 0 && i != 0 {
            println!("");
        }
        print!("{c} ");
    }

    let jap_len = hello_j.as_bytes().len();
    let cor_len = hello_j.as_bytes().len();

    println!("\n\nWith index bytes:");
    println!("{}\n{}", &hello_j[0..jap_len], &hello_k[0..cor_len]);
}

fn main() {
    word_count();
}