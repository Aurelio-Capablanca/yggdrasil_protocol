enum Operator<T> {
    Val(T),
    Sum(Box<Operator<T>>, Box<Operator<T>>),
    Multiply(Box<Operator<T>>, Box<Operator<T>>),
    Division(Box<Operator<T>>, Box<Operator<T>>),
    Subtract(Box<Operator<T>>, Box<Operator<T>>)    
}