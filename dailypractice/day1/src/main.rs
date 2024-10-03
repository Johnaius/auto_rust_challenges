fn main() {
    // **Challenge: **Given a number as an input, print out every integer from 1 to that number. However, when the integer is divisible by 3, print out “Fizz”; when it’s divisible by 5, print out “Buzz”; when it’s divisible by both 3 and 5, print out “Fizz Buzz”.

    //for loop that prints each number from 1 to n
    // 1..n+1 means our range starts at 1 and goes up to but does not include n+1.  Rust's range syntax (a..b) is exclusive of the upper bound (b).
    // i takes on the value in range each iteration.  
    fizzbuzz(15);
}


fn fizzbuzz(n : i32) {
    for i in 1..n+1{
        if i % 3 == 0 && i % 5 ==0{
            println!("{}","FizzBuzz");
        } else if i % 5 == 0{
            println!("{}","Buzz");
        } else if i % 3 == 0 {
            println!("{}","Fizz");
        }else{
            println!("{}", i);
        }
    }
}

