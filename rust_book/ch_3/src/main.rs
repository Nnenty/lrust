fn main() {
    let mut number = String::new();

    println!("Enter integer num:");
    std::io::stdin()
        .read_line(&mut number)
        .expect("Error with read");

    let number: u32 = number.trim().parse().expect("Error with convert");

    if number % 4 == 0 {
        println!("{number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("{number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("{number} is divisible by 2");
    } else {
        println!("{number} is not divisible by 4, 3, or 2");
    }

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    for_loop(array);
}
fn multiply(num: u32, mlt: u32) -> (u32, u32, u32) {
    print_mlt(num, mlt);

    (num, mlt, num * mlt)
}
fn print_mlt(num: u32, mlt: u32) {
    println!("{num} * {mlt} = {}", num * mlt);
}
fn i_mlt_two(count: u32) -> u32 {
    let mut iteration: u32 = 0;

    loop {
        iteration += 1;

        if iteration == count {
            break;
        }
    }
    iteration * 2
}
fn loop_label(nums: u32) {
    let mut count = 0;

    'counting_up: loop {
        println!("iteration = {}", count);

        let mut back: u32 = 10;

        loop {
            println!("back = {}", back);
            if back == 9 {
                break;
            }
            if count == nums {
                break 'counting_up;
            }
            back -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count)
}
fn for_loop(arr: [i32; 5]) {
    for (i, element) in arr.iter().enumerate() {
        println!("{} = {element}", i + 1);
    }
}
