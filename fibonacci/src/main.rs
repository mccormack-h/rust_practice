use std::io;

fn main() {
    println!("Enter the nth number in the Fibonacci sequence you want to see:");
    
    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("Readline failed");

    let num : i32 = num.trim().parse().expect("Parse failed");

    fibonacci_calc(num)
}

fn fibonacci_calc(i : i32)
{
    let mut n1 = 0;
    let mut n2;
    let mut end = 1;
    let mut x = 1; 
    while x < i
    {
        n2 = n1;
        n1 = end;
        end = n1 + n2;
        x+=1;
    }
    println!("The {i} Fibonnaci number is {}", end);
}
