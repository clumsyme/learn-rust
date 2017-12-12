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
    // in this way we cannot edit it
    take_reference(&mores);
    println!("after take reference, mores is {}", mores);

    // how about we want to edit it?
    let mut mus = String::from("I can be borrowed");
    borrow_mutable(&mut mus);
    println!("after borrow and mut, mus is {}", mus);

    let mut onlyone = String::from("hello");
    {
        // we can borrow it from lower scope,while the higher has no borrow now
        let borrowagain = &mut onlyone;
    }
    // and borrow in higher scope cause in this scope it's not borrowed yet
    let notonlyone = &mut onlyone;
    // we can only borrow mutable in scope once, the followed code will not work
    // let butonlytwo = &mut onlyone;
    // {
    //     let scopeborrow = &mut onlyone;
    // }
    // in fact, once we have a borrow, either mut or unmut,
    // we cannot borrow mut once again
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
    // some_string.push_str(", waoo"); we cant editable a borrow variable
    println!("got reference: {}", some_string);
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn borrow_mutable(some_string: &mut String) {
    some_string.push_str(", waoo");
}

fn copy_it(some_num: i32) {
    println!("{}", some_num);
}

// fn cant_return_ref() -> &String {
//     let s = String::from("ok");
//     &s
// } after the function call, s is gone, the refer may cause a dangling pointer, it's *forbidden*


fn can_return() -> String {
    let s = String::from("ok");
    s
}