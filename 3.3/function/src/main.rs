fn main() {
    println!("Hello, world!");

    another_function();
    print_int(5);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    print_int(y);

    print_int(five());

    print_int(add_one(five()));
}

fn another_function() {
    println!("Another function!");
}

fn print_int(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn add_one(x: i32) -> i32 {
    x + 1
}