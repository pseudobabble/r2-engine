extern crate dimensioned;
extern crate nom;

use dimensioned::si::{self, Meter, Second, M};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, space0};
use nom::multi::many0;
use nom::number::complete::double;
use nom::sequence::{delimited, preceded, terminated};
use nom::IResult;

use dimensioned::traits::Dimensioned;

use super::types::*;

fn parse_length(input: &str) -> IResult<&str, Dimensioned> {
    println!("reached parse_length {}", input.clone());

    // TODO: none of this is very nice, differentiate unit families better

    // https://docs.rs/nom/latest/nom/branch/fn.alt.html
    let (input, unit_alias) = delimited(
        tag("["),
        alt((
            tag("meters"),
            tag("meter"),
            tag("m"), // longest to shortest!!
            tag("kilometers"),
            tag("kilometer"),
            tag("km"),
        )),
        tag("]"),
    )(input)?;
    println!("parsed unit alias {}", unit_alias.clone());

    let dimension = match unit_alias {
        "m" | "meter" | "meters" => Meter * 1,
        "km" | "kilometer" | "kilometers" => Meter * 1000.0,
        _ => panic!("Unsupported unit alias {}", unit_alias),
    };

    Ok((input, dimension))
}

fn parse_number(number: &str) -> IResult<&str, AstNode> {
    println!("reached parse_number {}", number.clone());
    let (input, number) = double(number)?;
    println!("parsed number {}", number.clone());

    let (input, dimension) = parse_dimension(input)?;
    println!("parsed dimension {:#?}", dimension.clone());

    Ok((
        input,
        AstNode::Double {
            value: number,
            dimension: dimension,
        },
    ))
}

fn parse_name(name: &str) -> IResult<&str, AstNode> {
    println!("reached parse_name {}", name.clone());
    let (input, name) = alpha1(name)?;

    Ok((input, AstNode::Name(name.to_string())))
}

fn parse_operator(input: &str) -> IResult<&str, &str> {
    println!("reached parse_operator {}", input.clone());
    Ok(alt((
        terminated(preceded(space0, tag("+")), space0),
        terminated(preceded(space0, tag("-")), space0),
        terminated(preceded(space0, tag("*")), space0),
        terminated(preceded(space0, tag("/")), space0),
        terminated(preceded(space0, tag("^")), space0),
    ))(input)?)
}

fn parse_expression(input: &str) -> IResult<&str, AstNode> {
    println!("reached parse_expression {}", input.clone());

    let (input, _) = tag("(")(input)?;
    let (input, lhs) = alt((parse_number, parse_name, parse_expression))(input)?;
    let (input, operator) = parse_operator(input)?;
    let (input, rhs) = alt((parse_expression, parse_name, parse_number))(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((
        input,
        AstNode::Expression {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            operation: match operator {
                "+" => BinaryOperation::Add,
                "-" => BinaryOperation::Subtract,
                "*" => BinaryOperation::Multiply,
                "/" => BinaryOperation::Divide,
                "^" => BinaryOperation::Power,
                _ => panic!("Unsupported binary operation {}", operator),
            },
        },
    ))
}

fn parse_variable(input: &str) -> IResult<&str, AstNode> {
    println!("reached parse_variable {}", input.clone());
    let (input, name) = parse_name(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, expr) = terminated(alt((parse_number, parse_expression)), char(';'))(input)?;

    Ok((
        input,
        AstNode::Variable {
            name: Box::new(name),
            expr: Box::new(expr),
        },
    ))
}

pub fn parse_line(input: &str) -> IResult<&str, Vec<AstNode>> {
    many0(preceded(space0, parse_variable))(input)
}

#[test]
fn test_parse_number() {
    assert_eq!(
        parse_number("11e-1[m]"),
        Ok((
            "",
            AstNode::Double {
                value: 1.1,
                dimension: Dimension::Length { unit: Unit::Meter }
            }
        ))
    );
    assert_eq!(
        parse_number("1[meter]"),
        Ok((
            "",
            AstNode::Double {
                value: 1.0,
                dimension: Dimension::Length { unit: Unit::Meter }
            }
        ))
    );
    assert_eq!(
        parse_number("1.1[km]"),
        Ok((
            "",
            AstNode::Double {
                value: 1.1,
                dimension: Dimension::Length {
                    unit: Unit::Kilometer
                }
            }
        ))
    );
    assert_eq!(
        parse_number("9999999.987654[m]"),
        Ok((
            "",
            AstNode::Double {
                value: 9999999.987654,
                dimension: Dimension::Length { unit: Unit::Meter }
            }
        ))
    );
}

#[test]
fn test_parse_name() {
    assert_eq!(
        parse_name("test"),
        Ok(("", AstNode::Name("test".to_string())))
    );
    assert_eq!(
        parse_name("test"),
        Ok(("", AstNode::Name("test".to_string())))
    );
}

#[test]
fn test_parse_variable() {
    assert_eq!(
        parse_variable("test = 1.2[m];"),
        Ok((
            "",
            AstNode::Variable {
                name: Box::new(AstNode::Name("test".to_string())),
                expr: Box::new(AstNode::Double {
                    value: 1.2,
                    dimension: Dimension::Length { unit: Unit::Meter }
                })
            }
        ))
    );

    assert_eq!(
        parse_variable("var = -2[kilometers];"),
        Ok((
            "",
            AstNode::Variable {
                name: Box::new(AstNode::Name("var".to_string())),
                expr: Box::new(AstNode::Double {
                    value: -2.0,
                    dimension: Dimension::Length {
                        unit: Unit::Kilometer
                    }
                })
            }
        ))
    );
}

#[test]
fn test_parse_expression() {
    assert_eq!(
        parse_expression("(2[km] / 2[m])"),
        Ok((
            "",
            AstNode::Expression {
                operation: BinaryOperation::Divide,
                lhs: Box::new(AstNode::Double {
                    value: 2.0,
                    dimension: Dimension::Length {
                        unit: Unit::Kilometer
                    }
                }),
                rhs: Box::new(AstNode::Double {
                    value: 2.0,
                    dimension: Dimension::Length { unit: Unit::Meter }
                })
            }
        ))
    );

    assert_eq!(
        parse_expression("((2[m] / 2[km]) + (4[km] * 4[m]))"),
        Ok((
            "",
            AstNode::Expression {
                operation: BinaryOperation::Add,
                lhs: Box::new(AstNode::Expression {
                    operation: BinaryOperation::Divide,
                    lhs: Box::new(AstNode::Double {
                        value: 2.0,
                        dimension: Dimension::Length { unit: Unit::Meter }
                    }),
                    rhs: Box::new(AstNode::Double {
                        value: 2.0,
                        dimension: Dimension::Length {
                            unit: Unit::Kilometer
                        }
                    })
                }),
                rhs: Box::new(AstNode::Expression {
                    operation: BinaryOperation::Multiply,
                    lhs: Box::new(AstNode::Double {
                        value: 4.0,
                        dimension: Dimension::Length {
                            unit: Unit::Kilometer
                        }
                    }),
                    rhs: Box::new(AstNode::Double {
                        value: 4.0,
                        dimension: Dimension::Length { unit: Unit::Meter }
                    })
                })
            }
        ))
    );
}

#[test]
fn parse_variable_expression() {
    assert_eq!(
        parse_variable("var = (2[m] / 2[km]);"),
        Ok((
            "",
            AstNode::Variable {
                name: Box::new(AstNode::Name("var".to_string())),
                expr: Box::new(AstNode::Expression {
                    operation: BinaryOperation::Divide,
                    lhs: Box::new(AstNode::Double {
                        value: 2.0,
                        dimension: Dimension::Length { unit: Unit::Meter }
                    }),
                    rhs: Box::new(AstNode::Double {
                        value: 2.0,
                        dimension: Dimension::Length {
                            unit: Unit::Kilometer
                        }
                    })
                })
            }
        ))
    );

    assert_eq!(
        parse_variable("var = ((2[m] * 3[kilometers]) * (4[meters] + 5[km]));"),
        Ok((
            "",
            AstNode::Variable {
                name: Box::new(AstNode::Name("var".to_string())),
                expr: Box::new(AstNode::Expression {
                    operation: BinaryOperation::Multiply,
                    lhs: Box::new(AstNode::Expression {
                        operation: BinaryOperation::Multiply,
                        lhs: Box::new(AstNode::Double {
                            value: 2.0,
                            dimension: Dimension::Length { unit: Unit::Meter }
                        }),
                        rhs: Box::new(AstNode::Double {
                            value: 3.0,
                            dimension: Dimension::Length {
                                unit: Unit::Kilometer
                            }
                        }),
                    }),
                    rhs: Box::new(AstNode::Expression {
                        operation: BinaryOperation::Add,
                        lhs: Box::new(AstNode::Double {
                            value: 4.0,
                            dimension: Dimension::Length { unit: Unit::Meter }
                        }),
                        rhs: Box::new(AstNode::Double {
                            value: 5.0,
                            dimension: Dimension::Length {
                                unit: Unit::Kilometer
                            }
                        }),
                    })
                })
            }
        ))
    );
}

#[test]
fn parse_variables_and_abstract_expressions() {
    assert_eq!(
        parse_line("x = (2[m] * 2[kilometer]); y = 1[km]; z = (x + y);"),
        Ok((
            "",
            vec![
                AstNode::Variable {
                    name: Box::new(AstNode::Name("x".to_string())),
                    expr: Box::new(AstNode::Expression {
                        operation: BinaryOperation::Multiply,
                        lhs: Box::new(AstNode::Double {
                            value: 2.0,
                            dimension: Dimension::Length { unit: Unit::Meter }
                        }),
                        rhs: Box::new(AstNode::Double {
                            value: 2.0,
                            dimension: Dimension::Length {
                                unit: Unit::Kilometer
                            }
                        })
                    })
                },
                AstNode::Variable {
                    name: Box::new(AstNode::Name("y".to_string())),
                    expr: Box::new(AstNode::Double {
                        value: 1.0,
                        dimension: Dimension::Length {
                            unit: Unit::Kilometer
                        }
                    })
                },
                AstNode::Variable {
                    name: Box::new(AstNode::Name("z".to_string())),
                    expr: Box::new(AstNode::Expression {
                        operation: BinaryOperation::Add,
                        lhs: Box::new(AstNode::Name("x".to_string())),
                        rhs: Box::new(AstNode::Name("y".to_string()))
                    })
                },
            ]
        ))
    );
}
