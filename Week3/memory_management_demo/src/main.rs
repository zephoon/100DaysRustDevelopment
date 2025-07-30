use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("🧠 Memory Management Demo in Rust");

    // ownership example
    let s1 = String::from("Ownership Example");
    let s2 = s1;
    println!("Ownership: transferred: {}", s2);

    // Borrowing
    let s3 = String::from("Borrowing Example");
    let s4 = &s3;
    println!("AFter borrow: {}", s3); // still accessible

    // mutable borrowing
    let mut s4 = String::from("Hello");
    mutate_demo(&mut s4);
    println!("AFter mutation: {}", s4);

    // lifetime
    let result;
    let a = String::from("abcd");
    {
        let b = String::from("xyz");
        result = longest(&a, &b);
        println!("Longest string: {}", result);
    }

    // Box (heap allocation)
    let boxed = Box::new(42);
    println!("Boxed value: {}", boxed);

    // Rc (reference counting pointer)
    let rc_val = Rc::new(String::from("Shared"));
    let rc_clone = Rc::clone(&rc_val);
    println!("Rc value: {}, {}", rc_val, rc_clone);
    println!("Rc count: {}", Rc::strong_count(&rc_val));

    // RefCell (interior mutability)
    let cell = RefCell::new(100);
    *cell.borrow_mut() += 50;
    println!("RefCell value: {}", cell.borrow());
}

fn borrow_demo(data: &String) {
    println!("📥 Borrowed: {}", data);
}

fn mutate_demo(data: &mut String) {
    data.push_str(" World");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
