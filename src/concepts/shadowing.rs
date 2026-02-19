use crate::table_output;

// =========================
// Shadowing Sample
// =========================
// shadowing
// calculate value of a outer and inner
pub fn shadowing() {
    let a = 5;
    let a = a + 1;

    let inner_result = {
        let a = a * 2;
        format!("Inner scope: the value of a is {}", a)
    };

    let final_result = format!("{}\nOuter Scope: the value of a now is {}", inner_result , a);

    table_output!("Shadowing (move)", final_result)
}


// shadowing
// change the type of y
pub fn change_type() {
    let _y = 10; // underscore = intentionally unused
    
    let y = "hello";

    let result = format!("the type of y is now string, {}", y);
    table_output!("change type shadowing", result);
}

/*
Note: pagdating sa shadowing type.
pwede mag palit ng type.

Note: na yung underscore ay para
sa mga unused variables para iwas
warning logs sa terminal.
*/