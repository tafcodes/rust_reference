use std::io;
//Rust has some things from the stlib which are already in
//scope, referred to collectively as "The Prelude"
//io isn't one of these things.
use rand::Rng;
//this is a Trait, which I will be learning about soon.
use std::cmp::Ordering;

fn main() {
    println!("Guess numbers till you get it.");
    println!("Input guess: ");
    let mut guess = String::new();
    //vars are immutable by default.  
    //mut is a TODO ??????? WHAT IS IT?
    //mut makes them mutable obviously, but what _is_ 'mut'?

    //Rust has primitives, like 'str', which String uses.  

    let secret = rand::thread_rng().gen_range(1..101);
    println!("Secret is {}", secret);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Shoulda typed numbers");
    //this is called __shadowing__, so I assume that I can still get
    //to either 'guess' depending on whether the type needed is a String
    //or a u32
    


    match guess.cmp(&secret){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("Winner winner")
    }

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

    println!("You guessed: {}", guess);

}
