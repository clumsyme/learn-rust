fn main() {
    let x = 3;

    if x < 5 {
        println!("x is less than 5");
    } else {
        println!("nope");
    }

    let y = if true { 1 } else { 0 };
    println!("y is {}", y);

    let mut n = 0;
    loop {
        n = n + 1;
        println!("n = {}", n);
        if n == 10 {
            break;
        }
    }

    while n < 20 {
        n = n + 1;
        println!("n = {}", n);
    }

    for n in (1..10).rev() {
        println!("{}", n);
    }
}
