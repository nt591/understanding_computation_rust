struct BinaryOp {
    left: SimpleType,
    right: SimpleType
}
enum SimpleType {
    Number(i64),
    Add(Box<BinaryOp>),
    Multiply(Box<BinaryOp>),
}

trait Reducible {
    fn inner_reduce(&self) -> i64;
    fn reduce(&self) -> SimpleType;
}

impl std::fmt::Display for SimpleType {
    fn fmt(&self,  f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpleType::Number(val) => write!(f, "{}", val),
            SimpleType::Add(val) => write!(f, "{} + {}", val.left, val.right),
            SimpleType::Multiply(val) => write!(f, "{} * {}", val.left, val.right),
        }
    }
}

impl Reducible for SimpleType {
    fn inner_reduce(&self) -> i64 {
        match self {
            SimpleType::Number(val) => *val,
            SimpleType::Add(val) => val.left.inner_reduce() + val.right.inner_reduce(),
            SimpleType::Multiply(val) => val.left.inner_reduce() * val.right.inner_reduce(),
        }
    }
    fn reduce(&self) -> SimpleType {
        SimpleType::Number(self.inner_reduce())
    }
}

fn main() {
    let m1 = SimpleType::Multiply(Box::new(BinaryOp {
        left: SimpleType::Number(4),
        right: SimpleType::Number(8),
    }));
    let add = SimpleType::Add(Box::new(BinaryOp {
        left: SimpleType::Number(10),
        right: m1,
    }));

    println!("{}", add);
    println!("Value: {}", add.reduce());
    println!("Hello, world!");
}
