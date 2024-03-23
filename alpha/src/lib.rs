pub struct Value<'a>(pub &'a str);

pub fn consume<'a>(_: impl Into<Value<'a>>) {}

#[test]
fn foo() {
    let data = beta::produce("");
    consume(data);
}
