use crate::box_deref::box_deref_main;
use crate::drop_trait::drop_trait_main;
use crate::interior_mutability::interior_mutability_main;
use crate::reference_counter::rc_main;
use crate::reference_cycle_memory_leak::reference_cycle_memory_leak_main;
use crate::preventing_reference_cycle::preventing_reference_cycle_main;

mod box_deref;
mod drop_trait;
mod reference_counter;
mod interior_mutability;
mod reference_cycle_memory_leak;
mod preventing_reference_cycle;

fn main() {
    // box_deref_main();
    // drop_trait_main();
    // rc_main();
    // interior_mutability_main();
    // reference_cycle_memory_leak_main();
    preventing_reference_cycle_main();
}
