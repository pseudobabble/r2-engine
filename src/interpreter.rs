use super::types::*;

use std::clone::Clone;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::*;

#[derive(Debug, Clone)]
pub struct Memory<T: Clone> {
    instructions: Vec<Vec<AstNode>>,
    pub memory: HashMap<String, DimensionedValue<T>>,
}

pub trait Interpreter<T: Clone> {
    fn new(instructions: Vec<Vec<AstNode>>) -> Self;
    fn run(&mut self) -> ();
    fn evaluate(&self, expression: AstNode) -> DimensionedValue<T>;
    fn evaluate_expression(
        &self,
        operation: BinaryOperation,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    ) -> DimensionedValue<T>;
}

impl Interpreter<f64> for Memory<f64> {
    fn new(instructions: Vec<Vec<AstNode>>) -> Self {
        Memory {
            instructions: instructions,
            memory: HashMap::new(),
        }
    }

    fn run(&mut self) -> () {
        for line in &self.instructions {
            for variable in line {
                println!("\nCalculating {:#?}", variable.clone());
                let name = match variable {
                    AstNode::Variable { name, expr: _expr } => match *name.clone() {
                        AstNode::Name(name) => name,
                        _ => panic!("Variable name should be of type AstNode::Name"),
                    },
                    _ => panic!("Variable should be of type AstNode::Variable"),
                };

                let dimensioned_value = match variable {
                    AstNode::Variable {
                        name: _name,
                        expr: expression,
                    } => self.evaluate(*expression.clone()),
                    _ => panic!("Variable should be of type AstNode::Variable"),
                };

                println!("\nStoring result {:#?}", dimensioned_value.clone());
                self.memory.insert(name, dimensioned_value);
                println!("=================================\n\n");
            }
        }
    }

    fn evaluate(&self, expression: AstNode) -> DimensionedValue<f64> {
        let value = match expression {
            AstNode::Name(name) => self.memory[&name],
            AstNode::Double { value, dimension } => DimensionedValue::<f64> { value, dimension },
            AstNode::Expression {
                operation,
                lhs,
                rhs,
            } => self.evaluate_expression(operation, lhs, rhs),
            _ => panic!("Expression should be of type AstNode::Expression or AstNode::Double"),
        };

        value
    }

    fn evaluate_expression(
        &self,
        operation: BinaryOperation,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    ) -> DimensionedValue<f64> {
        let lhs_value = match *lhs {
            AstNode::Name(name) => self.memory[&name],
            AstNode::Double { value, dimension } => DimensionedValue::<f64> { value, dimension },
            AstNode::Expression {
                operation,
                lhs,
                rhs,
            } => self.evaluate_expression(operation, lhs, rhs),
            _ => panic!("Expression should be of type AstNode::Expression or AstNode::Double"),
        };

        let rhs_value = match *rhs {
            AstNode::Name(name) => self.memory[&name],
            AstNode::Double { value, dimension } => DimensionedValue::<f64> { value, dimension },
            AstNode::Expression {
                operation,
                lhs,
                rhs,
            } => self.evaluate_expression(operation, lhs, rhs),
            _ => panic!("Expression should be of type AstNode::Expression or AstNode::Double",),
        };

        match operation {
            BinaryOperation::Add => lhs_value + rhs_value,
            BinaryOperation::Subtract => lhs_value - rhs_value,
            BinaryOperation::Multiply => lhs_value * rhs_value,
            BinaryOperation::Divide => lhs_value / rhs_value,
        }
    }
}

#[test]
fn test_interpreter() {
    let i = Interpreter::<f64>::new(vec![vec![AstNode::Variable {
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

    println!("{:#?}", i.memory);
}
