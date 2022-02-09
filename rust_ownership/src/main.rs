fn main() {
    let how_long_is_hello = {
        let s = "hello";
        //That is a string literal, and it's stuck onto the stack.
        //it only exists within this block.  that's pretty convenient
        //for memory management
        s.len()
    };
    //inside the above block we make a thing but the only memory that
    //is used after the block is a single usize on the stack.

    println!("Hello is {} chars", how_long_is_hello);

    //We can do the same thing with heap-allocated resources:
    let how_long_2 = {
        //You can't make a string like this, because there's no
        //implicit casting
        //let s: String = "This is a bit longer and I am curious";
        //rather is has to be:
        let s = String::from("this is a bit longer and I'm curious how many chars");

        //also, I believe you could do this:
        let s3 = "heres a thing".to_string();

        //so, String has a method to get one from a str,
        //and str has a method to make a String out of it.

        s3.len()
    };
    println!("2ndstr is {} chars", how_long_2);

    //What happens if I try to declare a mutable str
    let mut s = "oh fudge";
    //s = (s + "you can't do this");
    s = "new str"; //but you can reassign the str type s to point
                   //to a different str.

    //this is in contrast to a String,
    let mut allocated_string = String::from("oh nice");
    allocated_string += ", you can do this";
    //allocated_string += '.'; //you can't do this, because that's a char
    allocated_string += "."; //
    allocated_string.push_str("\nAnd you can do it explicitly.");
    println!("allocated_string is: {}", allocated_string);

    //now I want to see what it looks like when I try to use
    //a var that's been 'invalidated' by rusts ownership mechanism

    let a2 = allocated_string;

    //below doesn't work.  We get an error: "borrow of moved value"
    //it is nice enough to show the definition of the old one.
    //println!("after a2, allocated_string is: {}", allocated_string);

    //essentially, there is no "shallow copy"
    //I kind of like that.
    //deep copy is accomplished with .clone()

    let a3 = a2.clone(); //this is a method that is actually simple.

    //if you want to know how a thing will behave, I think you may be able
    //to check whether it implements the Copy trait or the Drop trait

    //how these things behave with functions is interesting...
    //things passed by value (things which impl. the Copy trait)
    //will indeed be copied into the func
    //things passed by ref (things which impl. the Drop trait)
    //will pass "ownership".  So "a valid ptr" is "ownership"?
    //(the old ref is invalidated.)
    //This prevents you from passing something by reference, accidentally
    //modifying it and borking stuff.

    //stack vars, vars that are passed by value, primitives, types
    //which implement the "Copy" trait will behave like this:

    let stack_var: u8 = 255;
    let result_of_halving = half(stack_var); //will infer f32
    println!("halved is : {}", result_of_halving);

    //when it comes to heap-allocated resources:
    //You have a choice.  You can either give the function a .clone()
    //of your heap-allocated resource, using 2x the memory,
    //OR
    //you can give the function the thing by reference, and lose
    //the reference on the call-side, getting it back again on the
    //function's completion.  This forces you to make an assignment
    //and consider therefore the fact that the underlying heap
    //data may have changed!

    //let's do the deep-copy, clone, double-allocation first:
    let heap_string = String::from("This lives on the heap.");
    //below works, but then 'heap_string' is invalid.
    println!("halved is: {}", split_string_in_half(heap_string));
    //println!("original was: {}", heap_string);
    //above commented line does NOT work, because heap_string was
    //moved in the previous line, so we can't borrow it.
    //heap_string was invalidated.
    let heap_string_2 = String::from("This also lives on the heap.");
    println!(
        "2halved is: {}",
        split_string_in_half(heap_string_2.clone()),
    );
    //and, now we still have heap_string_2:
    println!("heap_string_2 still lives: {}", heap_string_2);

}

fn split_string_in_half(s: String) -> String {
    let half_idx = s.len() / 2;
    let half_s = &s[..half_idx];
    //note that the above is a slice of BYTES, not ~chars~,
    //and chars aren't exactly real in UTF-8 anyway

    return String::from(half_s);
}

fn half(i: u8) -> f32 {
    //my 'i' is a COPY of the passed value, existing on MY stack.
    //you can tell because if you look at a stack trace, you'd see
    //both "stack_var" from the main() frame, *AND* i in this frame.

    //you can ALSO tell this, because u8 implements the Copy trait.
    //you can't check at runtime if something implements a given trait.
    //You can probably do something at compile time to tell which might
    //be faster than running off to the documentation to check, but
    //I believe you can at least always check the documentation.

    i as f32 / 2.0
}
