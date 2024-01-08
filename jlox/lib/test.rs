use jlox_lib::Scanner;

#[test]
fn verify_left_paren() {
    let res = jlox_lib::run("(((!=");
    assert!(res.is_ok());
}

#[test]
fn check_scanner() {
    let input = "((()))!=";
    let scanner = Scanner::new(input);
    let tokens = scanner.tokens();
    assert_eq!(tokens.len(), 8);
}

#[test]
#[should_panic(expected = "Failed to parse some tokens")]
fn throws_on_illegal_inputs() {
    let input = "((()🦰";
    let scanner = Scanner::new(input);
    let tokens = scanner.tokens();
}
