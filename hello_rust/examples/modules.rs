#![allow(unused)]

mod foo {
    pub fn print() {
        println!("foo");
    }
}

pub mod my {
    // firstly import foo
    // use 'super' keyword to go one above each time
    // use super::foo;
    
    // pub fn print_foo() {
    //     foo::print();
    // }

    pub fn print() {
        println!("rust");
    }
    
    fn private_print() {
        println!("private");
    }

    pub mod a {
        use super::super::foo;
    
        pub fn print_foo() {
            foo::print();
        }

        pub fn print() {
            println!("a");
        }

        pub struct S {
            pub id: u32,
            name: String
        }

        pub fn build(id: u32) -> S {
            S {
                id,
                name: "".to_string()
            }
        }
    }
}

fn main() {
    my::print();
    my::a::print();

    // let s = my::a::S {
    //     id: 0,
    //     name: "Struct".to_string()
    // };

    let s = my::a::build(1);
    // my::print_foo();
    my::a::print_foo();

}