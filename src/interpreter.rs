use super::types::*;

use uom::lib::ops::{Add, Div, Mul, Sub};
use uom::si::f64;
use uom::si::f64::*;
use uom::si::length::{kilometer, meter};
use uom::si::time::second;
use uom::si::velocity::{kilometer_per_second, meter_per_second};

use std::collections::HashMap;

pub struct Interpreter {
    instructions: Vec<Vec<AstNode>>,
    memory: HashMap<String, f64>,
}

// https://github.com/iliekturtles/uom/issues/391
impl Interpreter {
    pub fn new(instructions: Vec<Vec<AstNode>>) -> Interpreter {
        Interpreter {
            instructions: instructions,
            memory: HashMap::new(),
        }
    }

    pub fn run(self) -> () {
        for line in self.instructions {
            for variable in line {
                let dimensioned_value = self.evaluate(variable.expression);
                self.memory.insert(variable.name, dimensioned_value);
            }
        }
    }

    fn evaluate(self, expression: AstNode) -> impl Add + Sub + Mul + Div {
        let value = match expression {
            AstNode::Double {
                value: value,
                dimension: dimension,
            } => self.evaluate_value(value, dimension),
            AstNode::Expression {
                operation: operation,
                lhs: lhs,
                rhs: rhs,
            } => self.evaluate_expression(operation, lhs, rhs),
        };
    }

    fn evaluate_expression(
        self,
        operation: BinaryOperation,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    ) -> impl Add + Sub + Mul + Div {
        let lhs_value = match *lhs {
            AstNode::Double {
                value: value,
                dimension: dimension,
            } => self.evaluate_value(value, dimension),
            AstNode::Expression {
                operation: operation,
                lhs: lhs,
                rhs: rhs,
            } => self.evaluate_expression(operation, lhs, rhs),
        };

        let rhs_value = match *rhs {
            AstNode::Double {
                value: value,
                dimension: dimension,
            } => self.evaluate_value(value, dimension),
            AstNode::Expression {
                operation: operation,
                lhs: lhs,
                rhs: rhs,
            } => self.evaluate_expression(operation, lhs, rhs),
        };

        match operation {
            BinaryOperation::Add => lhs_value + rhs_value,
            BinaryOperation::Subtract => lhs_value - rhs_value,
            BinaryOperation::Multiply => lhs_value * rhs_value,
            BinaryOperation::Divide => lhs_value / rhs_value,
        }
    }

    fn evaluate_value(self, value: f64, dimension: Dimension) -> impl Add + Sub + Mul + Div {
        match dimension {
            Dimension::Length { unit: unit } => match unit {
                Unit::Meter => Length::new::<meter>(value),
                Unit::Kilometer => Length::new::<kilometer>(value),
            },
        }
    }
}

#[test]
fn test_interpreter() {
    let i = Interpreter::new(vec![vec![AstNode::Variable {
        name: Box::new(AstNode::Name("var".to_string())),
        expr: Box::new(AstNode::Expression {
            operation: BinaryOperation::Divide,
            lhs: Box::new(AstNode::Double {
                value: 2.0,
                dimension: Dimension::Length { unit: Unit::Meter },
            }),
            rhs: Box::new(AstNode::Double {
                value: 2.0,
                dimension: Dimension::Length {
                    unit: Unit::Kilometer,
                },
            }),
        }),
    }]]);
}
