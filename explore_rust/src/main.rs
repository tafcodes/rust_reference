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

    let c = "c";
    let ch = 'c';



    println!("Type of double-quoted char is inferred as {}",
       std::any::type_name_of_val(c));
    println!("Type of single-quoted char is inferred as {}",
        std::any::type_name_of_val(&ch));
}