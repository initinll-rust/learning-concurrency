mod multi_threaded;

use multi_threaded::{
    concurrency_create_threads, 
    concurrency_move_ownership,
    concurrency_thread_channel,
    concurrency_shared_state
};

fn main() {
    concurrency_create_threads::create_new_threads();
    concurrency_move_ownership::move_ownership();
    concurrency_thread_channel::message_passing_between_threads();
    concurrency_shared_state::try_mutex_single_thread();
    concurrency_shared_state::share_mutex_multi_thread();
}
