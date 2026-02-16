use crate::table_output;

/***LOCAL VARIABLES***/

// 1. shadowing
pub fn shadowing() {
    let a = 5;
    let a = a + 1;
    
    /* inner scope */
    let inner_result = {
        let a = a * 2;
        format!("Inner scope: the value of a is {}", a)
    };

    let final_result = format!("{}\nOuter Scope: the value of a now is {}", inner_result , a); // value 6

    table_output!("Shadowing", final_result)
}

// 2. immutable borrowing
pub fn borrowing() {
    let s1 = String::from("Rust");

    let len = calculate_len(&s1);

    println!("s1 is {}, len is {}", s1, len);
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

// 3. immutable borrowing string
pub fn borrowing_string() {
    let s = String::from("hello");
    {
        let s1 = &s;
        println!("borrowing the s to s1 {}", s1);
    }
    
    println!("owner is s {}", s)
}
