//!
//! 8章 構造体
//!
mod name_field_type;
mod tuple_unit_type;
mod constant;
mod customer;
mod member;
fn main() {
    // name_field_type::generate();    // インスタンスの生成
    // tuple_unit_type::generate_tuple();
    // tuple_unit_type::generate_unit();
    // constant::use_constant();
    // constant::use_new();
    // customer::use_method();
    // member::use_method();
    // customer::use_debug();
    customer::use_clone();
}
