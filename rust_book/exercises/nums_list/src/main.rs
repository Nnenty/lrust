#[derive(Debug)]
struct List {
    val: Vec<i32>,
}

use std::collections::HashMap;

fn main() {
    let list = vec![
        65, 2, 3, 1, 8, 5, 3, 7, 2, 9, 7, 7, 7, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    ];

    let mut an_list = List::from(list);

    println!("Average = {}", an_list.average());
    println!("Median = {}", an_list.median());
    println!("Mode = {}", an_list.mode_list());
}
impl List {
    fn from(list: Vec<i32>) -> Self {
        List { val: list }
    }
    fn average(&self) -> i32 {
        let mut average = 0;

        for i in &self.val {
            average += i;
        }
        average = average / (self.val.len() as i32);

        average
    }
    fn median(&mut self) -> i32 {
        self.val.sort();

        let ret = self.val.len();

        (ret / 2) as i32
    }
    fn mode_list(&self) -> i32 {
        let mut list = HashMap::new();
        let mut max: i32 = 0;
        let mut max_num = 0;

        for num in &self.val {
            let count = list.entry(num).or_insert(0);

            *count += 1;
            if *count > max {
                max = *count;
                max_num = *num;
            }
        }

        max_num
    }
}
