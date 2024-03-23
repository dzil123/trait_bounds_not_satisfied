use alpha::Value;

pub fn produce() -> Value {
    Value
}

pub fn foo() {
    let data = produce();
    alpha::consume(data);
}
