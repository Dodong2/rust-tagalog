pub fn print_table(title: &str, result: String) {
    println!("\n+{}+", "-".repeat(title.len() + 2));
    println!("| {} |", title);
    println!("+{}+", "-".repeat(title.len() + 2));

    for line in result.lines() {
        println!("| {} |", line);
    }

    println!("+{}+\n", "-".repeat(title.len() + 2));
}

#[macro_export]
macro_rules! table_output {
    ($title:expr, $result:expr) => {
     crate::util::table_util::print_table($title, format!("{}", $result))
    };
}
