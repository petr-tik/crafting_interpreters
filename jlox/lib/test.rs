#[test]
fn verify_left_paren() {
    let res = jlox_lib::run("(((!");
    assert!(res.is_ok());
}
