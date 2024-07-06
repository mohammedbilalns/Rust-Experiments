use std::fmt 

fn main() {
    struct_example() 
}

fn struct_example(){

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }
    impl Person {
        fn say_hello(&self) {
            println!("Hello my name is {}!",self.name); 
        }
    }
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };
    let bob = Person{
        name: String::from("Bob"),
        ..alice //copied age from alice 
    };

    alice.say_hello();
    println!("{} is {}  years old.", alice.name, alice.age);


}


