fn main() {
    let t: i32 = "21".parse().expect("what is this");
    println!("Hello, {}!", t);

    let x = 57u8;
    println!("{}", x);

    let f = 3.14;
    let f2: f32 = 3.1415;
    println!("f: {}, f2: {}", f, f2);

    // operate
    let added = f2 + f;
    println!("added: {}", added);

    let tup: (i32, i32, f32) = (1, 222, 3.14);
    let (one, two, three) = tup;
    let one_alt = tup.0;
    println!("one: {}, one_alt: {}", one, one_alt);

    let arr = [1, 3, 5, 6, 8];
    println!("arr[0]: {}", arr[0]);
}
