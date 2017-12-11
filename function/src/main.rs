fn main() {
    println!("Hello, world!");

    another_function();

    let y = {
        let x = 3;
        x + 1
    };
    let x = 9 + 7;

    println!("x is {}", x);

    println!("y is {}", y);

    println!("power(8): {}", power(8));
}

fn another_function() {
    println!("Another called!");
}

fn power(n: i32) -> i32 {
    n * n
}
