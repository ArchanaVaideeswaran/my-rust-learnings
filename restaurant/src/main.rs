pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("hello from hosting");
            seat_at_table();
        }

        fn seat_at_table() {
            println!("table reserved");
            super::serving::take_order();
            super::serving::take_order();
            super::serving::take_payment();
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("order taken");
            serve_order();
        }

        fn serve_order() {println!("order served");}

        pub fn take_payment() {println!("payment received");}
    }
}


fn main() {
    println!("Hello, world!");
    front_of_house::hosting::add_to_waitlist();
}
