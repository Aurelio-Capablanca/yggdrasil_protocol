pub enum Expression<T>{
    VAR(T),
    OR(Box<Expression<T>>, Box<Expression<T>>),
    AND(Box<Expression<T>>, Box<Expression<T>>),
    NOT(Box<Expression<T>>),
    BIGGER(Box<Expression<T>>, Box<Expression<T>>),
    SMALLER(Box<Expression<T>>, Box<Expression<T>>)
}