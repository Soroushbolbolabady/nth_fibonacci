use std::io;



fn main() {
   
    

    println!("Enter a number: ");
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Something Wrong Happend");


    let inp: usize = inp.trim().parse().expect("You have to inter a number");

    
    if inp < 1{
        println!("Invalid Number");
    }else if inp == 1{
        println!("The first number in Fibonacci is : 1");
    }else{
        let res = generate_fibonacci(inp);
        println!("The {inp}th in fibonacci is: {res}");
    }

}

fn generate_fibonacci (x: usize) -> usize {
    let mut first = 0;
    let mut second = 1;
    let mut new = 0;


    let mut i = 2;
    while i <= x {
        new = first + second;
        first = second;
        second = new;

        i += 1;

    }

    new
}
