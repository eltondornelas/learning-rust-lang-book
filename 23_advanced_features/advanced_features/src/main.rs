use crate::unsafe_rust::unsafe_rust_main;
use crate::advanced_traits::advanced_traits_main;
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

mod unsafe_rust;
mod advanced_traits;
mod advanced_types;
mod advanced_functions_and_closures;
mod macros;


fn main() {
    // unsafe_rust_main();
    // advanced_traits_main();
    // advanced_types::advanced_types_main();
    // advanced_functions_and_closures::advanced_functions_and_closures_main();
    macros::macros_main();
}
