// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a hint.


mod delicious_snacks {
    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }

    use self::fruits::{PEAR, APPLE};
    use self::veggies::{CUCUMBER, CARROT};

    pub fn print_snacks() {
        println!(
            "favorite snacks: {} and {}",
            PEAR,
            CUCUMBER
        );
    }
}

fn main() {
    delicious_snacks::print_snacks();
}


