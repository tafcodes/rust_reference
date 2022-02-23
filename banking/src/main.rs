enum UsState {
        Alabama,
        Alaska,
        Arkansas,
        Pennsylvania,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<UsState>)
}

struct ChangePurse<> {
    //coinpurse to which coins can be added but never removed
    //and if you collect a quarter from all 4 states, you'll get 
    //a notification when you do, and upon future status queries.
    owner: String,
    contents: Vec<Coin>,
    completed_state_quarter_collection: bool
}



enum Bill {
    United_States,
    European_Union,
    Zimbabwe
}



enum Bill {
    One(Country),

}

struct Billfold {
    owner: String,
    contents: Vec<Bill>,

}

impl ChangePurse<> {

    fn create(owner: String) -> ChangePurse<> {
        ChangePurse {
            owner,
            contents: Vec::new(), //type will be inferred????
            completed_state_quarter_collection: false
        }
    }

    fn add_coin(&self, c: Coin) {
        //let my_contents=&self.contents;
        self.contents.push(c);
    }

    fn value_in_cents(&self) -> i32 {
        let mut accum = 0;    
        for coin in &self.contents {
            accum += match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(maybe_state) => 25
            }
        }
        accum
    }

    fn status(&self) {
        println!("{}'s purse has {} cents",
         self.owner,
         self.value_in_cents());
        if self.completed_state_quarter_collection {
            println!("Congratulations, you've collected quarters from all 4 states");
        } else {
            println!("Keep trying to find those state quarters!")
        }
    }
}

fn main() {
    let mut my_currency = ChangePurse::create(String::from("taf"));
    my_currency.add_coin(Coin::Penny);
    my_currency.add_coin(Coin::Quarter(None));
    my_currency.add_coin(Coin::Dime);
    my_currency.status()
}

