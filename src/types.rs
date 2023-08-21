use std::ops::{Add, Div, Mul, Sub};

use uom::si::f64::Length;
use uom::si::length::{kilometer, meter};
use uom::si::Quantity;

//type DimensionedValue<T> = Quantity<T, uom::si::SI<f64>, f64>;
trait Bind {}
impl Bind for Quantity<Length, uom::si::SI<f64>, f64> {}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Print(Box<AstNode>),
    Double(Box<dyn Bind>),
    Name(String),
    Expression {
        operation: BinaryOperation,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Variable {
        name: Box<AstNode>,
        expr: Box<AstNode>,
    },
}
