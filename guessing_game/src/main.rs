use std::io;
//Rust has some things from the stlib which are already in
//scope, referred to collectively as "The Prelude"
//io isn't one of these things.
use rand::Rng;
//this is a Trait, which I will be learning about soon.
use std::cmp::Ordering;

fn main() {
    println!("Guess numbers till you get it.");
    
    //vars are immutable by default.  
    //mut is a TODO ??????? WHAT IS IT?
    //mut makes them mutable obviously, but what _is_ 'mut'?

    //Rust has primitives, like 'str', which String uses.  

    let secret = rand::thread_rng().gen_range(1..101);
    //This type is inferred because later we compare it to guess.


    println!("Secret is {}", secret);

    loop {
        let mut guess = String::new();
        println!("Input guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            //expect is a way of unwrapping results.

        let guess: u32 = match guess.trim().parse() {
            //we can also explicitly unwrap results if we need
            Ok(num) => num, //this pattern is an Ok variant which contains a val
            Err(_) => continue
            //in this case, the error handling is not to panic.
        };
        //this is called __shadowing__, so I assume that I can still get
        //to either 'guess' depending on whether the type needed is a String
        //or a u32

        match guess.cmp(&secret){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("Winner winner");
                break;
            }
        }
    };
    println!("You won, so now you get to leave.")

    /*another way to write that would be like:
    let stdin_handle = std::io::stdin();
    match stdin_handle.read_line(&mut guess) {
        //match is a switch/case?
        //Ok/Error are possible variants the Result type can have
        Ok(n) => {
            //This is a lambda?
        }
        Err(error) => println!("{}", error);

        //Result has some kind of "must_use" flag attached to it
        //which means you get compiler warnings if you call a func
        //which returns a result but then don't ever use the result
        //That's neat!
    }
    */

}
