// =========================
// basics
// =========================
pub fn print() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{}, days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number=1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);
}

/*
Note: Ang tawag sa {} ay fmt::Display.

Lahat ng Built-in Types
(integers, strings, booleans)
ay automatic na pwedeng i-print.
Ang kailangan lang i-define "manually"
ay yung mga Custom Types
(yung mga ikaw mismo ang gumawa, tulad ng struct).

automatic:
println!("{0}, this is {1}", "Alice", "Bob");

need defined:
struct Student(i32);
*/


