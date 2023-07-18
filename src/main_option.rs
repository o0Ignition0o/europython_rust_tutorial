enum Direction {
    North(u8),
    South,
    East,
    West(Person),
}

struct Person {
    name: String,
    age: u8,
}

enum Maybe {
    Present(Person),
    Absent
}

fn maybe_person() -> Maybe {
    return None;
    Some(Person {
        name: "Jeremy".to_string(),
        age: 34,
    })
}

fn main() {
    let maybe = maybe_person(); // .unwrap();
    let the_person = match maybe {
        Some(p) => {
            println!("I have a person");
            p
        }
        None => {
            println!("oh no");
            Person {
                name: "".to_string(),
                age: 42,
            }
        }
    };
    // let jeremy = Person {
    //     name: "Jeremy".to_string(),
    //     age: 34
    // };

    // let Person {age, name} = jeremy;
    // cant use jeremy anymore
    // dbg!(name);

    // if let Direction::North = my_direction {
    //     println!("to the north!");
    // }

    // let my_direction = Direction::West(jeremy);

    // match my_direction {
    //     Direction::North(42) => {
    //         println!("FORTY TWO");
    //     }
    //     Direction::North(length) => {
    //         println!("{} kilometers to the north!", length);
    //     }
    //     Direction::West(p) => {
    //         println!("{} is going west", p.name);
    //     }
    //     _ => {
    //         println!("not the north");
    //     }
    // }

    // let maybe_something = Some(42);

    // dbg!(maybe_something);
}
