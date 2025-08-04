

#[derive(Debug)]
pub enum Operator<T> {
    Val(T),
    Sum(Box<Operator<T>>, Box<Operator<T>>),
    Multiply(Box<Operator<T>>, Box<Operator<T>>),
    Division(Box<Operator<T>>, Box<Operator<T>>),
    Subtract(Box<Operator<T>>, Box<Operator<T>>),
    Or(Box<Operator<T>>, Box<Operator<T>>),
    And(Box<Operator<T>>, Box<Operator<T>>),
    Not(Box<Operator<T>>),
    Equals(Box<Operator<T>>, Box<Operator<T>>),
    NotEquals(Box<Operator<T>>, Box<Operator<T>>),
    Greater(Box<Operator<T>>, Box<Operator<T>>),
    GreaterEqual(Box<Operator<T>>, Box<Operator<T>>),
    Less(Box<Operator<T>>, Box<Operator<T>>),
    LessEqual(Box<Operator<T>>, Box<Operator<T>>),
}
