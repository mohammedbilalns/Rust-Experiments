pub mod helpers;

fn main() {
    let result =  helpers::namehelpers::get_full_name("bilal", "ns");
    println!("{result}");
    let age_plusfive = helpers::privatefns::get_age_plus_5(10);
    println!("{}",age_plusfive);
}


