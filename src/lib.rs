mod front_of_house;

// this lib.rs module is further contained in an implicit module known as a root crate
mod back_of_house {
    // making an enum public exposes all of its variants
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        // toast field is public
        pub toast: String,
        // seasonal fruit is private and allows no access or operations on it
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn what_fruit(&self) {
            println!("{}", self.seasonal_fruit)
        }
    }

    // Call a function in the ancestor using super
    pub fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

fn serve_order() {}

pub fn eat_at_restaurant() {
    //Absolute Path
    crate::back_of_house::fix_incorrect_order();

    //Cook something with a relatve path
    back_of_house::fix_incorrect_order();

    // Order breakfast with some sourdough
    let mut meal = back_of_house::Breakfast::summer("Sourdough");

    // Oh no they have Commo Ciabatta let's get that instead
    meal.toast = String::from("Commo Ciabatta");
    println!("I'd like {} toast please", meal.toast);

    // Show the fruit
    meal.what_fruit();

    let salad = back_of_house::Appetizer::Salad;

    let soup = back_of_house::Appetizer::Soup;

    println!("Salad: {}, Soup: {}", salad, soup);
}
