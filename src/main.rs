use std::io;



fn main() {
   
    

    println!("Enter a number: ");
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Something Wrong Happend");


    let inp: usize = inp.trim().parse().expect("You have to inter a number");

    
    if inp != 2 {
        let a = generate_fibonacci(inp);
        println!("The {inp}th number in Fibonacci is : {a}");
    }else{
        println!("The 2nd number in Fibonacci is : 1");
    }

}

fn generate_fibonacci (x: usize) -> usize {
    let mut first = 0;
    let mut second = 1;
    let mut new = 0;


    let mut i = 3;
    while i <= x {
        new = first + second;
        first = second;
        second = new;

        i += 1;

    }

    new
}
