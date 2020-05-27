#[cfg(test)]

/// `parse()` handles input of the form `2d6`
#[test]
fn parse_handle_long_form() {
    use crate::dice::Dice;
    let cmd: Dice = Dice::new(2, 6);
    assert_eq!(cmd, "2d6".parse().unwrap());
}

#[test]
fn parse_handle_short_form() {
    use crate::dice::Dice;
    let cmd: Dice = Dice::new(1, 6);
    assert_eq!(cmd, "6".parse().unwrap());
}
