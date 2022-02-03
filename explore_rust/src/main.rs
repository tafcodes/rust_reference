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
//shadows can be different types.  This is different than mutation,
//it's just the name being reused.  

//This is kind of nice, because in a function it means you can keep using
//a name that you like, but then when execution leaves that scope you 
//are assured that, if it wasn't marked 'mut', it didn't change.

