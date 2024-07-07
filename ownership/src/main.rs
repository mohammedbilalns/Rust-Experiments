fn main(){

    let foo = 42;
    let  f = &foo; // pointer that points to foo , grabs ownership and read permissions from foo
     //foo +=2;  error! since f is immutable and f is borrowed by f 
    // f += 2;  error! since f is a pointer 
     //*f += 2 ; error! since reference f is immutable 
    let mut  bar = *f;// New memmory with the value of foo
    //*f =65;   // error; since reference f is immutable 
    println!("{foo}"); 
    println!("{f}"); // Prints the new value of foo 
    println!("{bar}"); // no change since data is copied from foo 
    bar = 54; //
    println!("{f}"); // prints the value of reference f  
    // f is dropped here 
    println!("{bar}");// prints the modified value of bar 
    println!("{foo}");// prints the value of f 

}
