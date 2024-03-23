pub struct Value;

pub fn consume(_: impl Into<Value>) {}

#[test]
fn foo() {
    let data = beta::produce();
    consume(data);
}
