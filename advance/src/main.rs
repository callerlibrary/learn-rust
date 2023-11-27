mod async_ts_mod;
mod macro_ts_mod;

use advance::*;
use async_ts_mod::*;
use macro_ts_mod::*;

use futures::executor::*;
fn main() {
    // lifetime::life_cycle();
    // closure::closure();
    // thread_ts::thread_run();
    // thread_ts::thread_move();

    err_ts::os_and_ts();
    err_ts::or_else_ts();
    err_ts::ang_then_ts();
    err_ts::filter_ts();

    macro_ts::ts();

    // 引入vec_ts宏
    let v = vec_ts![1, 2, 3];
    println!("{:?}", v);

    async_ts::ts();
    let future = async_ts::do_something();
    block_on(future);

    block_on(async_ts::async_main());
}
