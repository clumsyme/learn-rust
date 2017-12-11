fn main() {
    let t = "21".parse().expect("what is this");
    println!("Hello, world!");

    //     2 |     let t = "21".parse().expect("what is this");
    //   |         ^
    //   |         |
    //   |         cannot infer type for `_`
    //   |         consider giving `t` a type
}
