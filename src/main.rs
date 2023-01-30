pub mod tuple;
pub mod array_slice;
pub mod enum_match;
pub mod pointer_ref_match;
pub mod match_struct;
fn main() {
    //tuple::match_tuple();
    //array_slice::match_array();
    //enum_match::match_enum();
    //pointer_ref_match::match_pointer_ref();
    match_struct::match_struct();
}