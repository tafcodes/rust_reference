//Is it a type?
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

//You can have tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
//These won't be interchangeable by accident later.

//Unit Struct?
//It just makes... a word?  Sounds like I will learn about this with Traits.
struct AlwaysEqual;
//let subject = AlwaysEqual;
//I guess this is kind of like an enum with only 1 possible value,
//or perhaps more generally, a Symbol.


fn main() {
    //So we just put a tag there that says "user".  Mmk.
    let me = User {
        email: String::from("dev@taf.codes"),
        username: String::from("dev"),
        active: true,
        sign_in_count: 1
    };

    //structs have mutability as a property.  So we won't be able to do this:
    //  me.email = String::from("not@my_email.co");
    //The compiler error is very clear.  

    let mut anyone = User {
        email: String::from("first_identity@fastmail.com"),
        username: String::from("first_id"),
        active: false,
        sign_in_count: 0
    };

    //now, we could activate the user
    anyone.active = true;


    let you = build_user(String::from("you@your.net"), String::from("you"));
    let u2 = User {
        username: String::from("you2"),
        ..you  //this is a struct update shorthand.  Copies all the fields
        //from the other struct which aren't specified here
    };

    //Below will NOT work, since we've moved the values.  
    //  println!("Your email: {}", you.email);

    //Awkwardly though, this DOES work:
    println!("Your username: {}", you.username);
    println!("Your other username: {}", u2.username);
    //since we gave a new username, we never moved the old one.  
    //So...now we're left with a partially invalid struct.  That's kind of yuck.

    //...and, of course, the struct's fields which were datatypes that implement
    //the Copy trait are still available in the old semi-valid struct too...
    println!("Your active status: {}", you.active);

    //What if I want to look at the whole struct?
    //Can I print it?
    //    println!("You are like this: {}", u2);
    //no.

    //Let's do exactly as the compiler suggests - the macro lets us specify
    //different printers.  
    println!("You are like this: {:#?}", u2);
    //this printer relies on the Debug trait.  
    //We can derive the debug trait for User, or we could implement it.  
    //the compiler error is very helpful, it tells us how.  I will add the
    //annotation / "attribute" before the Struct def #[derive(Debug)]

    //Now, I want to know what _that_ is.  Is it a macro?  A compiler directive?

    //we'll figure that out later.  In the meantime, here's a macro to print 
    //with the debug printer faster:

    dbg!(&u2);

    //This macro takes ownership of the expression inside, and ALSO returns it
    //So this lets you stick it into the middle of things to do print debugging
    //without having to stick a print statement anywhere.

    //e.g.
    let u3 = User {
        active: dbg!(! u2.active),
        ..u2
    };
    //I like that.

    ///
    //Methods
    //
    
    //methods can be defd in terms of a struct, a trait, or an enum
    //let's define a Rectangle struct which has a method for area.  
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32
    }

    #[derive(Debug)]
    struct Square {
        size: u32
    }

    impl Rectangle {
      fn area(&self) -> u32 {
        self.width * self.height
    }
    //&self is, within an impl block, shorthand for self: &Self, where Self is
    //an alias for the type the Block is about.

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.width >= other.width && self.height >= other.height {
            return true
        }
        false
    }
   }
   //now, for things Typed as Rectangles, we can access a .area()

   impl Square {
    //I am curious about what happens when you take ownership of self rather
    //than borrowing it as you usually would.  I expect that it will invalidate
    //the variable that had held the struct previously, unless we return a
    //new struct.

    fn to_rectangle(&self) -> Rectangle {
        Rectangle {
            width: self.size,
            height: self.size
        }
    }

    fn as_rectangle(self) -> Rectangle {
        Rectangle {
            width: self.size,
            height: self.size
        }
    }
   }

   let rect1 = Rectangle {
    width: 30,
    height: 50
   };
   println!("Area of rect is {} sq px", rect1.area());

   let sq1 = Square { size: 20 };
   let r2 = sq1.to_rectangle();
   let r3 = sq1.as_rectangle(); // I believe this will kill sq1, as
   //it takes ownership of it, rather than borrowing it?
   //let's check
   //dbg(sq1)  //can't do it, sq1 is invalid after "sq1.as_rectangle()"
   //because that method takes ownership of the value.
   dbg!(&r3);  //can do this, and it looks how I expected.

   //you can have methods with the same name as fields.  This might be a getter
   //or it might not.  A getter might be useful because you could set some fields
   //to be private.

   //in this case, I've used Rectangle.width() to return true if there's a valid
   //(nonzero) width.
   dbg!(r3.width());

   //Rust will automatically dereference borrows when you're doing a method call
   //on a borrowed value.

   //let's be deliberately horrible now.
   dbg!(
    (Square {size: 30}).as_rectangle()
        .can_hold(
            &Square {size:20}.as_rectangle()
        )
    );

    dbg!(
    (Square {size: 20}).as_rectangle()
        .can_hold(
            &Square {size:30}.as_rectangle()
        )
    );

    //great, even my bad code works.

    

}

//OK, but is a struct a _type_?
fn build_user(email: String, username: String) -> User {
    //There's shorthand we can use here.
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}