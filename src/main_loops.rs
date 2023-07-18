fn main() {
    let mut i = 0;

    while i < 100 {
        // println!("{i}");
        i += 1;
    }

    for i in 0..=100 {
        println!("{i}");
    }

    loop {
        i += 1;
        if i % 2 == 0 {
            continue;
        }
        println!("{i}");
        if i > 100 {
            break;
        }
    }
}
