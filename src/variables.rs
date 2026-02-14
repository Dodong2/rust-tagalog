/***LOCAL VARIABLES***/

// 1. shadowing
pub fn shadowing() {
    let a = 5;
    let a = a + 1;
    
    {
        let a = a * 2;
        println!("the value of a now is {a}")
    }

    println!("the value of a now is {a}") // value 6
}


pub fn borrowing() {
    let s1 = String::from("Rust");

    let len = calculate_len(&s1);

    println!("s1 is {}, len is {}", s1, len);
}

fn calculate_len(s: &String) -> usize {
    s.len()
}


pub fn borrowing_string() {
    let s = String::from("hello");
    {
        let s1 = &s;
        println!("borrowing the s to s1 {}", s1);
    }
    
    println!("owner is s {}", s)
}

pub fn borrowing_capacity() {
    let c = String::from("Rust");

    let cap = calculate_cap(&c);

    println!("c is {}, capacity is {}", c, cap);
}

fn calculate_cap(s: &String) -> usize {
    s.capacity()
}