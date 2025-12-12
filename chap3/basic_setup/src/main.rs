use futures::{executor::block_on, future::join_all, join};
use std::{
    thread::{self, JoinHandle},
    time,
};
use async_std;

fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}

// [NOTE] `async` keyword used to create a future fn
/*
- can called by
    - JoinHandle<i8> = thread::spawn(|| do_something(1));
    - with `future` crate: .await, .join!, (block_on( a async future_method ))
*/

async fn do_some(num: i8) -> i8 {
    println!("num {} is running", num);
    let one_sec = time::Duration::new(2, 0);
    thread::sleep(one_sec);
    return 1;
}
fn main() {
    let now = time::Instant::now();
    // [NOTE] Sync functions call - one called - wait 2s - two called - waited 2s - three called - wait 2s -> result 6
    // let one =do_something(1);
    // let two = do_something(2);
    // let three = do_something(3);

    // println!("time elapsed {:?}", now.elapsed());
    // println!("result {}", one + two + three);

    // [NOTE] Spin off a thread for each function
    /*
    - create a JoinHandle which use thread::spawn closure call to do_something()
    - put thread into main thread by .join()
    - get the result by .unwrap()
    */
    // let thread_one: JoinHandle<i8> = thread::spawn(|| do_something(1));
    // let thread_two: JoinHandle<i8> = thread::spawn(|| do_something(2));
    // let thread_three: JoinHandle<i8> = thread::spawn(|| do_something(3));

    // let res_one = thread_one.join();
    // let res_two = thread_two.join();
    // let res_three = thread_three.join();

    // println!("time elapsed {:?}", now.elapsed());
    // println!(
    //     "result {}",
    //     res_one.unwrap() + res_two.unwrap() + res_three.unwrap()
    // )

    // [NOTE] Use async/await with `future` crate. (block_on( a async future_method ))

    // let future_one = do_some(1);
    // let out_come = block_on(future_one);

    // println!("time elapsed {:?}", now.elapsed());
    // println!("Here is the outcome: {}", out_come);

    // can also be extracted by using await: use `async` block: async { .await }
    // let future_two = async { return do_some(2).await };
    // let future_two = block_on(future_two);

    // println!("Here is the outcome: {}", future_two);
    // `await` can call flexible in async block
    // let future_three = async {
    //     let future_four = do_some(4).await;
    //     let future_five = do_some(5).await;
    //     return [future_four, future_five]
    // };
    // let future_three = block_on(future_three);

    // println!("time elapsed {:?}", now.elapsed());
    // println!("Here is the outcome: {:?}", future_three);

    // use futures::join have a `join!` marco function take in half by getting two futures to run at the same time. -> NOT WORKING. just make remove .await, use join! shorter but returned a tuple, not reduce time
    // let future_sametime_run = async {
    //     let future_four = do_some(4);
    //     let future_five = do_some(5);
    //     return join!(future_five, future_four)
    // };
    // let future_sametime_run = block_on(future_sametime_run);

    // println!("time elapsed {:?}", now.elapsed());
    // println!("[futures::join] result is tuple: {:?}", future_sametime_run);

    // [NOTE] crate async_std: create own async joins function to r
    let third_outcome = async {
        let mut futures_vec = Vec::new();
        let future_four = do_some(4);
        let future_five = do_some(5);
        futures_vec.push(future_four);
        futures_vec.push(future_five);

        let handles = futures_vec.into_iter().map(async_std::task::spawn).collect::<Vec<_>>();
        let result = join_all(handles).await;
        return result
    };

    let result = block_on(third_outcome);
    println!("time elapsed for join vec {:?}", now.elapsed());
    println!("Here the result {:?}", result);
}
