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
