fn main() {
   string_slices() 
}

fn string_slices(){
    let a = "hi  fsdf fsd fd fd";
    println!("{}",a.len());
    let first_word = &a[0..2];
    let second_word =&a[3..7];
    println!("{}",a.is_empty());

    println!("{},{}",first_word,second_word);
}
fn string_literals () {
    let a ="hi";
    let b: &'static str = "hi";
    println!("\0");
}
