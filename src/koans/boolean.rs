// Booleans can have two values, true or false.
// Two equal values will return true when compared with the == operator
#[test]
fn truth() {
    assert!(true == true);
}

// Likewise, two unequal values will return false when compared with ==
// The != operator can be used to return true for an inequality
#[test]
fn falsehood() {
    assert!(false != true);
}

// Strings can also be compared and will return a boolean
#[test]
fn string_equality() {
    assert!("Stuff" == "Stuff");
}

// Integers can be compared as long as they are of the same type
#[test]
fn int_equality() {
    let num: i8 = 5;
    assert!(num == 5);
}
