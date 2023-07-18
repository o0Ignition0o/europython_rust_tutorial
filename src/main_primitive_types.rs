fn main() {
    let my_boolean = false;
    let my_other_boolean: bool = true;
    dbg!(my_boolean);

    let my_age: u8 = 34;
    let my_age = 34_u128;
    let my_age_agein = 34_usize;

    let my_number = if my_age > 0 {
        println!("i have an age");
        42
    } else {
        println!("i dont");
        64
    };

    let my_tuple = (42, true, "hello");

    let my_array = [42u8, 43, 44];
    let my_vector = vec![42u8, 43, 44];

    let hello = "hello";
    let hello_string = String::from("hello");

    let mut my_boolean = false;
    my_boolean = true;
}
