use alpha::Value;

pub fn produce<'a>(val: &'a str) -> Value<'a> {
    Value(val)
}

pub fn foo() {
    let data = produce("");
    alpha::consume(data);
}
