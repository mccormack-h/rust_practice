use std::io;

fn main() {
    println!("Enter a temperature in F:");

    let mut f_temp = String::new();

    io::stdin().read_line(&mut f_temp).expect("Readline failed.");

    let f_temp : i32 = f_temp.trim().parse().expect("Parsing failed.");

    let c_temp = (f_temp - 32) as f32 / 1.8;

    println!("{f_temp} Fahrenheit is {c_temp:.3} Celsius");
}
