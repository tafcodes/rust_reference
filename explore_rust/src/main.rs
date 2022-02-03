fn main() {
    //variable shadowing time.

    let x = 5;

    let x = x + 1;

    { //apparently I can just have a block anywhere to separate scope
        let x = x * 2;
        println!("Value of X in inner scope is : {}", x);
    }

    println!("Value of X in outer scope is {}", x);

}


//ok, so what?

