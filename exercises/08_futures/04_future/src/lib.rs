//! TODO: get the code to compile by **re-ordering** the statements
//!  in the `example` function. You're not allowed to change the
//!  `spawner` function nor what each line does in `example`.
//!   You can wrap existing statements in blocks `{}` if needed.
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    {
        let non_send = Rc::new(1);
        println!("{}", non_send);
    }

    // * to solve the problem we moved the rest of the code to
    // * an inner scope and kept the yield out of it.
    // todo: why?
    yield_now().await;
}
