fn main() {
    // Automatic type inference 
    let mut  vec1 = vec!["fsd","fsd","slf"];
    // Explicit with type 
    let mut vec2 = Vec::<i32>::new();
    let mut vec3 = Vec::new();
    vec3.push(54.54);
    vec2.push(5);
    vec1.push("flsjdbf");
    // iterate in for 
    for i in vec1.iter(){
        println!("{i}");
    }
}
