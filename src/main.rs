enum SimpleType {
    Number {
        value: i64,
    },
    Add {
        left: Box<SimpleType>,
        right: Box<SimpleType>,
    },
    Multiply {
        left: Box<SimpleType>,
        right: Box<SimpleType>,
    },
}

trait Reducible {
    fn inner_reduce(&self) -> i64;
    fn reduce(&self) -> SimpleType;
}

impl std::fmt::Display for SimpleType {
    fn fmt(&self,  f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpleType::Number { value } => write!(f, "{}", value),
            SimpleType::Add { left, right } => write!(f, "{} + {}", left, right),
            SimpleType::Multiply { left, right } => write!(f, "{} * {}", left, right),
        }
    }
}

impl Reducible for SimpleType {
    fn inner_reduce(&self) -> i64 {
        match self {
            SimpleType::Number { value } => *value,
            SimpleType::Add { left, right } => left.inner_reduce() + right.inner_reduce(),
            SimpleType::Multiply { left, right } => left.inner_reduce() * right.inner_reduce(),
        }
    }
    fn reduce(&self) -> SimpleType {
        SimpleType::Number { value: self.inner_reduce() }
    }
}

fn main() {
    let m1 = SimpleType::Multiply {
        left: Box::new(SimpleType::Number { value: 4 }),
        right: Box::new(SimpleType::Number { value: 8 }),
    };
    let add = SimpleType::Add {
        left: Box::new(SimpleType::Number { value: 10 }),
        right: Box::new(m1),
    };

    println!("{}", add);
    println!("Value: {}", add.reduce());
    println!("Hello, world!");
}
