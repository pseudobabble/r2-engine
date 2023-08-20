use super::types::*;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Interpreter {
    instructions: Vec<Vec<AstNode>>,
    pub memory: HashMap<String, DimensionedValue>,
}

// https://github.com/iliekturtles/uom/issues/391

impl Interpreter {
    pub fn new(instructions: Vec<Vec<AstNode>>) -> Interpreter {
        Interpreter {
            instructions: instructions,
            memory: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> () {
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

    fn evaluate(&self, expression: AstNode) -> DimensionedValue {
        let value = match expression {
            AstNode::Double { value, dimension } => DimensionedValue { value, dimension },
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
    ) -> DimensionedValue {
        let lhs_value = match *lhs {
            AstNode::Double { value, dimension } => DimensionedValue { value, dimension },
            AstNode::Expression {
                operation,
                lhs,
                rhs,
            } => self.evaluate_expression(operation, lhs, rhs),
            _ => panic!("Expression should be of type AstNode::Expression or AstNode::Double"),
        };

        let rhs_value = match *rhs {
            AstNode::Double { value, dimension } => DimensionedValue { value, dimension },
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

    println!("{:#?}", i.memory);
}
