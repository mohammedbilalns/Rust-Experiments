use std::fmt; 

fn main() {
   static_ex() 
}


// Basic Struct example 
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

// Static and inference  example 
fn static_ex(){
    struct Rectangle{
        length: i32,
        width: i32
    }

    impl  Rectangle {
        fn new(length: i32,width: i32) -> Rectangle{
            Rectangle{ length, width}
        } // this is static 

        fn rectangle(side: i32) -> Rectangle {
            Rectangle{ length: side, width: side}
        } // this is static too 

        fn area(&self) -> i32{
            self.width * self.length
        } // this is inference 

        fn perimeter(&self) -> i32 {
            2 *(self.width + self.length)
        } // this is inference too 
    }


    let reat = Rectangle::new(43, 434);
    let re = Rectangle::area(&reat);
    println!("{}",re);
    println!("{}",reat.length);
    println!("{}",reat.width);


}

// tuple like structs
fn tuple_likestructs(){
    struct Location (i32,i32);
    let loc = Location(43,32);
    println!("{},{}", loc.0, loc.1);
}

//unit like structs it is rarely used 
fn unit_likestructs(){
    struct Marker;
    let m = Marker;


}

