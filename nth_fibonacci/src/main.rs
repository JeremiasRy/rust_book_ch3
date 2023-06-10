use std::io;

fn main() {
    println!("Tell me the nth fibonacci number you want:");

    let nth = get_input();

    println!("The {} fibonacci is: {}", nth, get_nth_fibonacci(nth));
}

fn get_input() -> i32 {
    let result: i32 = loop {
        let mut input: String = String::new();  
        
        io::stdin()
            .read_line(&mut input)
            .expect("What's going on???");

        let input: i32 = match input
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid value");
                    continue;
                }
            };
        break input;
    };
    result
}

fn get_nth_fibonacci(nth: i32) -> i32 {
    let mut first = 1;
    let mut second = 1;
    let mut middle_man;
    let mut count = 2;

    while count < nth {
        middle_man = first + second;
        first = second;
        second = middle_man;
        count += 1;
    }
    second
}
