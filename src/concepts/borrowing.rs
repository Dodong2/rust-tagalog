use crate::table_output;
/***BORROWING***/

// =========================
// IMMUTABLE Sample
// =========================
// immutable - string
// calculate the stack length of hello
fn calculate_len(s: &String) -> usize {
    s.len()
}


pub fn immutable_string() {
    let s1 = String::from("Hello");

    let len = calculate_len(&s1);

    let result = format!("s1 is {}, len is {}", s1, len);
    table_output!("immutable - String", result)
}


// immutable - number
// calculate the stack size of 10
fn calculate_stack(n: &i32) -> usize {
    std::mem::size_of_val(n)
}

pub fn immutable_number() {
    let n = 10;

    let size = calculate_stack(&n);
    
    let result = format!("size of {}, is {}", n, size);
    table_output!("immutable - number", result)
}


// =========================
// MUTABLE Sample
// =========================
// mutable - string
// extends the message (dagdag)
fn extends_message(some_string: &mut String) {
    some_string.push_str(", Carl Pogi welcome sa Rust.");
}

pub fn mutable_string() {
    let mut message = String::from("Hello");

    extends_message(&mut message);

    let result = format!("full message: {}", message);

    table_output!("mutable - string", result);
}




//mutable - number
// change the number into 50
fn change_number(num: &mut i32) {
    *num = 50;
}


pub fn mutable_number() {
    let mut number = 20;

    change_number(&mut number);
    
    let result = format!("before: the number is 20, now the number is {}", number);
    table_output!("mutable - number", result);
}


/* 
Note: hindi practical na gamitin si mutable
(&mut) sa simple numbers Kasi nga may 
Copy trait ang i32.

Note: sa mutable bawal siya mag palit ng 
types.dapat same types lang string kung 
string. integer kung integers, if string 
to integers. bawal yun kasi error nayun.

Results:
immutable - String: s1 is Hello, len is 5 
immutable - number: size of 10, is 4
mutable - string: full message: Hello, Carl Pogi welcome sa Rust.
mutable - number: before: the number is 20, now the number is 50
*/










