fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x=6;
    println!("The value of x is: {}", x);
    //this isn't going to work because x isn't tagged mut

    //consts do NOT have type inference.  Let's see what compiler msg
    //I get if I try to use const without specifying a type.

    //const hey = "Hey"; //this is a string literal, I believe, not a String
    //nice, clear and consistent error message "Provide a type for the const"
    //the compiler even infers "&str" for me, and proposes that I add it!

    //I'm curious though, what if I just call it a "str"?
    //(I'm assuming &str is a pointer to a raw str)
    //const hey: str = "Hey";
    //interesting.  It seems like consts have a Trait "sized",
    //and the compiler won't infer size at compile time...

    //I know the right thing to do is just:
    //const hey: &str = "Hey";
    //but can we type it as a "str" if it's a _variable_?

    //let heyv = "Hey";
    //it's unused.  I get a compiler warning for that.  Neat!
    //I can prefix it with a _ to avoid that warning.

    //let _heyv = "Hey";
    //nice.

    //but, back to the types.  What happens if I try to call it a str?

    //let _heyv: str = "Hey";
    //Okay.  The compiler makes it clear that the right side of the expr
    //is a "&str".  So it's a simple type mismatch.  
    //Subsequently, I *also* get a warning about the "Sized" Trait.  

    //So I want to know how to explicitly print types, because I
    //will get confused later and want to print the types of things
    //println!("A string literal is type: {}", std::any::type_name("literal"))
    //okay, that doesn't work.  type_name takes zero args?
    //off to the doc

    //ok, I'm going to leave that one alone for now until I understand
    //type-generic functions in rust.

    

}
