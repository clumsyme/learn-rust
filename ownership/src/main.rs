fn main() {
    let s = String::from("halo!");
    println!("{}", s);

    let mut ms = String::from("halo!");
    ms.push_str(" walde~");
    println!("{}", ms);

    let cms = ms.clone();
    println!("ms is {} and cms is {}", ms, cms);

    let mms = ms;
    // from now, ms is moved and shall not be used
    println!("mms is {}", mms);

    // but for variable in stack, no need to clone
    let x = "halo";
    let y = x;
    println!("x is {} and y is {}", x, y);

    println!("s is {}", s);
    take_ownership(s);
    // the following code will raise a "use of moved value" Error
    // println!("s is {}", s);

    let n = 8;
    copy_it(n);
    println!("n is {}", n);

    // get ownership
    let gs = give_ownership();
    println!("got gs as {}", gs);

    let feed = String::from("whao");
    let tgs = take_and_give_ownership(feed);
    println!("tgs is {}", tgs);

    // what if we need to return another value and keep use of the variable
    // we can return a tuple
    let mores = String::from("how long");
    let (mores, len) = take_and_give_more(mores);
    println!("{}'s length is {}", mores, len);

    // but is this not so convenient，we can use *reference*
    take_reference(&mores);
    println!("after take reference, mores is {}", mores);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn give_ownership() -> String {
    let s = String::from("got it");
    s
}

fn take_and_give_ownership(some_string: String) -> String {
    some_string
}

fn take_and_give_more(some_string: String) -> (String, usize) {
    let len = some_string.len();
    (some_string, len)
}

fn take_reference(some_string: &String) { // s is a reference to a String
    println!("got reference: {}", some_string);
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn copy_it(some_num: i32) {
    println!("{}", some_num);
}
