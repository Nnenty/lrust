use std::collections::HashMap;

fn main() {
    let mut company: HashMap<String, Vec<&str>> = HashMap::new();

    let mut python = vec!["bot", "python_love", "maxim"];
    let mut rust = vec!["main rustacean", "barbarik", "belyanka"];
    let mut c = vec!["ded", "inside", "#write_it_yourself(#wiy)"];

    python.sort();
    rust.sort();
    c.sort();

    company.entry(String::from("python")).or_insert(python);
    company.entry(String::from("rust")).or_insert(rust);
    company.entry(String::from("c")).or_insert(c);

    company.iter().for_each(|(dep, names)| {
        println!("\t{dep}: ");

        names.iter().for_each(|name| {
            print!("{name}, ");
        });

        println!("\n");
    });

    // println!("List of the company developers:");
    // for (dep, names) in &company {
    //     println!("\t{dep}: ");

    //     for name in names {
    //         print!("{name}, ");
    //     }

    //     println!("\n");
    // }
}
