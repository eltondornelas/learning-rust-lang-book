use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use using_threads_run_code_simultaneously::using_threads_run_code_simultaneously_main;
use using_message_passing_between_threads::using_message_passing_between_threads_main;
use shared_state_concurrency::shared_state_concurrency_main;

mod using_threads_run_code_simultaneously;
mod using_message_passing_between_threads;
mod shared_state_concurrency;

fn main() {
    // using_threads_run_code_simultaneously_main();
    // using_message_passing_between_threads_main();
    shared_state_concurrency_main();
}

