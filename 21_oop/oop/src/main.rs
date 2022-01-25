use crate::encapsulation::encapsulation_main;
use crate::oop_state_pattern::oop_pattern_main;
use crate::encoding_state_into_type_system_pattern::encoding_state_into_type_system_pattern_main;
use crate::traits::traits_main;

mod encapsulation;
mod traits;
mod oop_state_pattern;
mod encoding_state_into_type_system_pattern;

fn main() {
    // encapsulation_main();
    // traits_main();
    // oop_pattern_main();
    encoding_state_into_type_system_pattern_main();
}

