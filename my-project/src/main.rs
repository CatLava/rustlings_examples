mod front_house{
    // right now these are private
    // use pub ot expose and use
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seating() {}
    }

    pub mod serving{
        fn take_order() {}

        pub fn serve_order() {}



        fn pay_bill() {}
    }
}

mod back_house {
    // use structs and enums to develop This
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // appetizer enum

    // When the enum is public, all of its variants are public

    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_order() {
        cook_order();
        super::front_house::serving::serve_order();
    }

    fn cook_order() {
    }
}

pub fn eat_at_restaraunt() {
    front_house::hosting::add_to_waitlist();
    let mut meal = back_house::Breakfast::summer("rye");
    // change mind
    meal.toast = String::from("sour");
    println!("This toast: {}", meal.toast);

    let order1 = back_house::Appetizer::Soup;
}

fn main() {
    eat_at_restaraunt();
    println!("hi");
}
