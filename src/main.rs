enum SimpleType {
    Number { value: i64 },
    Add { left: Box<SimpleType>, right: Box<SimpleType> },
    Multiply { left: Box<SimpleType>, right: Box<SimpleType> },
}

trait Showable {
    fn show(&self) -> String;
}

trait Reducible {
    fn reduce(&self) -> i64;
}

impl Showable for SimpleType {
    fn show(&self) -> String {
        match self {
            SimpleType::Number { value }=> value.to_string(),
            SimpleType::Add { left, right }=> format!("{} + {}", left.show(), right.show()),
            SimpleType::Multiply { left, right }=> format!("{} * {}", left.show(), right.show())
        }
    }
}

impl Reducible for SimpleType {
    fn reduce(&self) -> i64 {
        match self {
            SimpleType::Number { value } => *value,
            SimpleType::Add  { left, right } => left.reduce() + right.reduce(),
            SimpleType::Multiply  { left, right } => left.reduce() * right.reduce(),
        }
    }
}

fn main() {
    let m1 = SimpleType::Multiply { left: Box::new(SimpleType::Number { value:  4 }), right: Box::new(SimpleType::Number { value:  8 }) };
    let add = SimpleType::Add { left: Box::new(SimpleType::Number { value:  10 }), right: Box::new(m1) };

    println!("{}", add.show());
    println!("Value: {}", add.reduce());
    println!("Hello, world!");
}
