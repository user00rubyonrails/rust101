fn main() {
    let test = "test".to_string();
    let test_func = || {
        println!("{}", test);
        return test + &String::from(" case")
    };
    let outcome = test_func();
    println!("{}", outcome)
}

