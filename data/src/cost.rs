pub struct Cost {
    currency: Currency,
    value: Value,
}

pub enum Currency {
    Dollar,
    Euro,
    Ruble,
}

pub struct Value(f64);