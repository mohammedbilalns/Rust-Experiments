use std::io;
use rand::seq::index;
use rand::Rng;
use std::io::{Write,BufReader,BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    enum_exam();  
}

// Random number generation 
fn random_number(){
    let random_num = rand::thread_rng().gen_range(1..=100);
    println!("Random : {random_num}");
}


//if and else 
fn if_and_else (){
    let age: i32 = 8;
    if (age >= 1) && (age <= 18) {
        println!("Important birthday");  
    } else if (age == 21) || (age ==50) {
        println!("IMportant birthday");
    } else if age >= 65 {
        println!("IMportant birthday");
    } else {
        println!("NOt an important birthday");
    }
}


// if else terinary 
fn if_terinary(){
    let  my_age: i32 = 47;
    let can_vote: bool = if my_age >= 18 {true} else {false};
    println!("can vote: {}",can_vote);
}

//match 

fn match_birthday(){
    let age = 8;
    match age {
        1..=18 => println!("Important birthday"),
        21 | 50 => println!("IMportant birthday"),
        65..=i32::MAX => println!("iMportant birthday"),
        _ => println!("Not an important birthday"), 
    };
}

// Compare 
fn compare_example(){
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("can vote"),
        Ordering::Equal => println!("You gained the right to vote")
        
    };
} 

// Array
fn array_example(){
    let arr_1= [1,3,34,5,45,45,87,8,78,45];
    println!("1st: {}",arr_1[0]);
    println!("length: {}",arr_1.len());
    let mut loop_idx = 0;
    loop {
        if arr_1[loop_idx] % 2 == 0 {
            loop_idx +=1;
            continue;
        }
        if arr_1[loop_idx] == 9 {
            break;
        }
        println!("val : {}",arr_1[loop_idx]);
        loop_idx += 1;
    }
}

// While loop 
fn while_loopexamp(){
    let arr_2 = [1,3,3,43,34,34,343,43,43,434];
    let mut loop_indx = 0;
    while loop_indx< arr_2.len() {
            println!("Arr : {}",arr_2[loop_indx]);
        loop_indx +=1
        
    }
}

// for loop 
fn for_loopexamp(){
    let arr_3 = [343,43,34433,43,434,343,43,43];
    for val in arr_3.iter()  {
        println!("val : {}",val);
        
    }
}
// Tuples 
fn tuple_examp(){
    let my_tuple: (u8,String,f64) = (43,"bilal".to_string(),54.54);

    println!("name: {}",my_tuple.1);
    let(v1,v2,v3) = my_tuple;
}

// Strings 
fn string_ex(){
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace(){
        println!("{}",word);
    }
    let st2: String = st1.replace("A", "Another");
    println!("{}",st2);

    let st3 = String::from("x r b h s k l l s l ");
    let mut v1:Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup(); // remove duplicates 
    for char in v1 {
        println!("{}",char);
    }
    let st4: &str = "Random String"; 
    let mut st5: String = st4.to_string();
    println!("{}",st5);
    let byte_arr1 = st5.as_bytes(); 
    let st6 = &st5[0..6];
    println!("String lenth: {}",st6.len());
    st5.clear();
    let st6 = String::from("Just some ");
    let st7 = String::from(" words ");
    let st8 = st6 + &st7;
    for char in st8.bytes(){
        println!("{}",char);
    }
}

// casting 
fn casting_example(){
    let int1: u8 = 5;
    let int2: u8 = 3;
    let int3: u32 = int1 as u32 + int2 as u32;   
}

//enums 
fn enum_exam() {
    enum Day {
        Monday , Tuesday, Wednesday, Thursday, Friday, Saturday , Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false    
            }
        }
    }
    let today:Day = Day::Monday;
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        _ => println!("It is not MOnday"),
    };

    println!("is today the weekend {}",today.is_weekend());
}

// Vectors 
fn vect_exam(){

    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4,5];
    vec2.push(5);
    println!("1st : {}",vec2[0]);
    match vec2.get(1) {
        Some(second) => println!("2nd: {}",second),
        None => println!("No second value"),
    }
    for i in  &mut vec2 {
        *i *=  2 ;

    }
        
}
