fn say_hi(name: String) {
    println!("Hi {}!", name);
}

fn say_hi_borrow(name: &String) {
    println!("Hi {}!", name);
}

fn say_hi_mutable_borrow(name: &mut String) {
    *name = format!("Hi how you doin?");
}

fn main() {
    let mut jeremy = "jeremy".to_string();
    say_hi_mutable_borrow(&mut jeremy);
    say_hi_borrow(&jeremy);
    dbg!(jeremy);
}
