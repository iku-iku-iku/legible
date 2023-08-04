use compiler::ast;

#[test]
fn test_block() {

    let b = crate::ast::Block::new("DSA".to_string());
    let s  = "DS".to_owned() + "F";
    println!("{:?}", b);

    // println!("Hello, world!");
}
