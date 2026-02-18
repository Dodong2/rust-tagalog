/* main */
mod variables;
mod references_borrowing;
mod util;
fn main() {
    /* variables */
    variables::shadowing();
    /* borrowings */
    references_borrowing::immutable_string();
    references_borrowing::immutable_number();
    references_borrowing::mutable_string();
    references_borrowing::mutable_number();
}
