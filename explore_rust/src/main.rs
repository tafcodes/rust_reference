fn main() {

    //types.  Scalar and compound.  

    //All the scalar types:
    let an_integer: u32 = 5;
    let a_float: f64 = 3.14159;
    let a_bool: bool = true ;
    let a_char: char = 'a';

    //integers:
    let vshort=255u8;
    //that works, since you can stick a typehint onto a literal, but
    //probably the below would be more proper:
    let vshort:u8 = 255;

    //we can specify in decimal, hex, oct, bin,
    let a: i64 = 1023;
    let a: i64 = 0xfff;
    let a: i64 = 0o777;
    let a: i64 = 0b00011001;
    //we can also freely add underscores to numeric literals
    let a_million = 1_000_000;
    let a_billion = 1_000_000_000;
    //this would be handy for a bitmap
    let bits: u8 = 0b0000_0110;
    println!("A billion looks like {}", a_billion);

    let a_double = 1.69; //will default to f64
    let another_dub = 1_000_000.60009f64;

    //there's only one kind of bool and it's a byte.

    //chars are ***four bytes***, which I believe makes them able
    //to hold *most* UTF-8 things, but ****NOT EVERYTHING****

    //let _: () = "c";
    //see note below,
    //the above expr is a &str
    //let _: () = 'c';
    //see note below,
    //the above expr is a char

    //Ok....so types don't really exist at runtime, therefore the only
    //way to figure out the type of an expression is to piss off the 
    //compiler by provoking a deliberate mismatch???

    //compound types

    let tuple: (i32, f64, u8) = (500, 12.8, 255);
    
    //let arr = [1, 2, 3.14, 50000000000000];
    //hoping this becomes a float64?
    //but it does not.  It thinks everything should be an int, because
    //the first item was?
    //so we would have to hint it like this:

    //let arr: [f64] = [1,2,3.14, 5000000000];
    //that doesn't work either, because it sees integer literals
    //and will NOT cast seamlessly to floats.  
    //so you actually have to do it like this:
    let arr = [1.0,2.0,3.14, 5000000000.0];
    //but if I want to mandate the type, 
    //I will also have to mandate the length

    let one_thousand_zeroes = [0;1000];
    //arrays are on the stack, not heap.
    //if you want them to be growable, you can use a vector
    //which will be on the heap

    let y = {
        //expressions can be blocks.
        //blocks can ofc have statements
        let x = 3;
        x+1
    };
    //will evaluate to y being 4


    //later on I will make this a module that exports some things
    println!("{}",fib("50"));
}

///calling a macro is an expression?

//when you don't use a semicolon, it's only an expression,
//not a statement
fn sfib(n: &str) -> &str {
    //now we'll shadow 


    //I want to see the compiler warnings that I get, so
    //let n: u32 = n.trim().parse(); //predict this will be a type mismatch
    //because it will return a result that will NOT be implicitly used.
    //nice!

    //so it will actually have to be like this:
    let n: u32 = match n.trim().parse() {
            //we can also explicitly unwrap results if we need
            Ok(num) => num, //this pattern is an Ok variant which contains a val
            Err(_) => panic!("Couldn't parse n to int")
            //in this case, the error handling is not to panic.
    };


    if n==0 || n==1 {
        return "1"
    }

    sfib(to_str(n -1)) + sfib(to_str(n-2))

}

//now I'm just being deliberately bad
fn to_str(x: u32) -> &str {
    let s: String = x.to_string();
    let ss: &str = &s;
}


fn fib(n: u64) -> u64 {
    if n == 0 {
        return 1
    }

    if n == 1 { 
        return 1
    }

    fib(n - 1) + fib(n - 2)
}
