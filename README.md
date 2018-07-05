# Rust-Calculator
HW # 1 for Rust Programming Class. The program implements a command-line calculator that can handle 4 functions.
It will be able to calculate the sum, product, gcd, and lcm of multiple numbers. The program will still exit
successfully if only given one value or just the operation.


# Implementation
For the implementation of the calculator I seperated the code into two parts. The first the main function will handle 
grabbing the arguments from the command line, parsing them into the correct data types and printing the results. The
second part of the code consisits of all the helper functions which will be called by main to calculate the correct
results.

## Main Function
The main function starts off by grabbing the arguments from the command line and seperating it into the operation that
is called and a vector of values that operation will work on. The vector consists of floating point numbers. There are
a few error tests which will check for correct operations and number of values. The function then uses a match statement
to determine which operation needs to be done and calls the correct helper function where the return value is finally
stored and printed out to the screen.

## Helper Functions
For the helper function that will calculate the result of the arguments passed in, they are mostly straight forward.
The addition and multiplication simply takes the arguments and adds/multiplys them from left to right and returns the 
final result. There are two gcd helper functions. The first gcd function calculates the gcd of two values using the
Euclidean algorithm in a iterative way. The seconds gcd function gcd multiple finds the gcd of all the arguments passed
in by applying the gcd helper functions to each argument left to right. The same process is used to find the lcm, there
is one function to calculate the lcm of two values and another to apply it to the arguments from left to right. 

# How it went?

# Testing
