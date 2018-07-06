use std::env;
use std::str::FromStr;

fn main() {
    //Grab arguments off of command line and place in to a Vector
    let arguments: Vec<String> = env::args().skip(1).collect();

    //Exit successfully if there are no numeric arguments and only an operation argument
    if check_args(&arguments) == 1 {
        return;
    }

    //The first item in the vector should be the operation, so grab it and place it in a variable
    let operation = &arguments[0];

    //copy over the numeric values from arguments vector and covert them to floating point values
    let mut values = Vec::new();
    for i in 1..arguments.len() {
        values.push(f64::from_str(&arguments[i]).expect("error in parse"));
    }

    //Match the first argument with what operation needs to be done
    //If it does not match any operation panic.
    let result = match operation.as_ref() {
        "sum" => addition(values),
        "product" => multiplication(values),
        "gcd" => gcd_multiple(values),
        "lcm" => lcm_multiple(values),
        _ => panic!("{} not a valid operation", operation),
    };

    //Print out result
    println!("{}", result)
}

//Function to check that there is atleast 1 argument passed in from the command line
//if there are no arguments then it will panic, if there are only 1 return 1, else just
//return
fn check_args(args: &Vec<String>) -> i32 {
    let mut ret = 0;
    if args.len() == 0 {
        panic!("Need to provide atleast one command line argument");
    }
    if args.len() == 1 {
        ret = 1;
    }
    ret
}

//Function that sums together all values in the vector from left to right. returns 0 if
//vector is empty
fn addition(values: Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for i in values {
        sum = sum + i;
    }
    sum
}

//Function that multiplies together all values in the vector from left to right, returns
//0 if vector is empty
fn multiplication(values: Vec<f64>) -> f64 {
    let mut product = 1.0;
    if values.len() > 0 {
        for i in values {
            product = product * i;
        }
    } else {
        product = 0.0;
    }
    product
}

//Finds the gcd of just two numbers using euclidean algorithm
fn gcd(mut n: f64, mut m: f64) -> f64 {
    assert!(n != 0.0 && m != 0.0);
    while n != m {
        if n > m {
            n = n - m;
        } else {
            m = m - n
        }
    }
    n
}

//Finds the gcd of multiple numbers by finding the gcd of the first two numbers, then finding the
//gcd of the next number with the gcd of the first two and so on
fn gcd_multiple(values: Vec<f64>) -> f64 {
    let mut answer = values[0];
    for i in 1..values.len() {
        answer = gcd(answer, values[i]);
    }
    answer
}

//Find lcm using the formula that involves the gcd of the two numbers
fn lcm(n: f64, m: f64) -> f64 {
    let gcd_answer = gcd(n, m);
    let answer = (n * m) / gcd_answer;
    answer
}

//Find the lcm of multiple numbers by finding the lcm of the first two numbers, then finding the
//lcm of the next number with the lcm of the first two and so on
fn lcm_multiple(values: Vec<f64>) -> f64 {
    let mut answer = values[0];
    for i in 1..values.len() {
        answer = lcm(answer, values[i]);
    }
    answer
}

//Tests for Homework 1 Calculator
#[test]
//This test will check that the calculator exits quietly when only 1 argument is provided
fn test1_arguments() {
    let mut args: Vec<String> = Vec::new();
    args.push("lcm".to_string());
    assert_eq!(check_args(&args), 1);
}

#[test]
#[should_panic]
//This test will check that the calculator panics when no arguments are given.
fn test_no_arguments() {
    let mut args: Vec<String> = Vec::new();
    args.pop();
    check_args(&args);
}
#[test]
//There are 4 unit tests, which will test each of the operations (addition, multiplication, GCD,
//and LCM. In each unit test there will be 3 assertions to check when there are more than 2 values,
//only 2 values, and 1 valuepassed into the function.
fn test_sum() {
    let mut values = vec![2.0, 3.0, 4.0, 5.0];
    assert_eq!(addition(values), 14.0);
    values = vec![3.0, 4.0];
    assert_eq!(addition(values), 7.0);
    values = vec![2.0];
    assert_eq!(addition(values), 2.0);
}

#[test]
fn test_product() {
    let mut values = vec![2.0, 3.0, 4.0, 5.0];
    assert_eq!(multiplication(values), 120.0);
    values = vec![3.0, 4.0];
    assert_eq!(multiplication(values), 12.0);
    values = vec![2.0];
    assert_eq!(multiplication(values), 2.0);
}

#[test]
fn test_gcd() {
    let mut values = vec![30.0, 45.0, 75.0, 80.0];
    assert_eq!(gcd_multiple(values), 5.0);
    values = vec![30.0, 45.0];
    assert_eq!(gcd_multiple(values), 15.0);
    values = vec![2.0];
    assert_eq!(gcd_multiple(values), 2.0);
}

#[test]
fn test_lcm() {
    let mut values = vec![5.0, 2.0, 4.0, 10.0];
    assert_eq!(lcm_multiple(values), 20.0);
    values = vec![3.0, 4.0];
    assert_eq!(lcm_multiple(values), 12.0);
    values = vec![2.0];
    assert_eq!(lcm_multiple(values), 2.0);
}
