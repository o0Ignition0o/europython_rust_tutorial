fn is_adult(age: u8) -> bool {
    age >= 18
}

fn main() {
    let adult = is_adult(34);

    assert!(adult);

    let not_adult = is_adult(15);
    assert!(not_adult == false);
}

#[test]
fn is_adult_should_return_true_for_adult() {
    // Setup
    let my_age = 34;
    let expected_result = true;
    // Run
    let actual_result = is_adult(my_age);
    // Assert
    assert_eq!(expected_result, actual_result);
}

#[test]
fn is_adult_should_return_false_for_child() {
    // Setup
    let my_age = 4;
    let expected_result = false;
    // Run
    let actual_result = is_adult(my_age);
    // Assert
    assert_eq!(expected_result, actual_result);
}
