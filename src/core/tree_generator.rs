use crate::core::tokenization::Token;

#[derive(Debug)]
enum OperationTree<T> {
    Number(T),
    UnaryOperation{
        operation: Token,
        expression: Box<OperationTree<T>>,
    },
    BinaryOperation{
        left: Box<OperationTree<T>>,
        operation: Token,
        right: Box<OperationTree<T>>,
    }
}


