/* main */
mod basics;
mod concepts;
mod util;
fn main() {
    /* basics */
    basics::formatted_print::print();
    /* shadowing */
    concepts::shadowing::shadowing();  
    concepts::shadowing::change_type();
    /* borrowings */
    concepts::borrowing::immutable_string();
    concepts::borrowing::immutable_number();
    concepts::borrowing::mutable_string();
    concepts::borrowing::mutable_number();
}
