use std::io;

fn main() {
    let mut n = String::new();
    let mut count = 0;
    println!("Enter n");
    io::stdin()    
        .read_line(&mut n)    
        .expect("Failed to read line");    
    
    let n: u32 = n.trim().parse().expect("Please type a number!");
    let mut f_0 = 0;
    let mut f_1 = 1;
    while count < n {
        let temp = f_0;
        f_0 = f_1;
        f_1 = f_1 + temp;
        count += 1;
    };
    println!("{f_1}");

}
