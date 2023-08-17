use std::any::Any;

extern crate nom;
extern crate uom;

use uom::si::f32::*;
use uom::si::length::kilometer;
use uom::si::time::second;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char, digit1, space0};
use nom::combinator::map;
use nom::multi::many0;
use nom::number::complete::double;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::str::FromStr;

/// (average_wage_per_unit_calendar_time * (avoided_days_of_lost_due_to_anxiety + avoided_days_of_lost_due_to_depression))
///  * ((proportion_proxy_benefit_attributed_to_wellbeing_anxiety + proportion_proxy_benefit_attributed_to_wellbeing_depression) / 2)
///  * imp_employee_time_in_mental_wellbeing_programmes

/// [inputs]
/// average_wage = 12[USD]/4[month]
/// avoided_days = 3[days]
///
/// [calculation]
/// employee_mental_wellbeing_benefit[currency*time] = (average_wage_per_unit_calendar_time[currency/time] * (avoided_days_of_lost_due_to_anxiety[time] + avoided_days_of_lost_due_to_depression[time]))
///  * ((proportion_proxy_benefit_attributed_to_wellbeing_anxiety[float] + proportion_proxy_benefit_attributed_to_wellbeing_depression[float]) / 2)
///  * imp_employee_time_in_mental_wellbeing_programmes[time]

/// use fuel

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum UnaryOperation {
    Negate,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Print(Box<AstNode>),
    Double(f64),
    Name(String),
    UnaryOp {
        operation: UnaryOperation,
        expr: Box<AstNode>,
    },
    BinaryOp {
        operation: BinaryOperation,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Variable {
        name: Box<AstNode>,
        expr: Box<AstNode>,
    },
}

// pub fn parse(input: &str) -> IResult<&str, AstNode> {
//     parse_expression(input)
// }

// fn parse_expression() {}

fn parse_number(number: &str) -> IResult<&str, crate::AstNode> {
    let (input, number) = double(number)?;

    Ok((input, AstNode::Double(number)))
}

fn parse_name(name: &str) -> IResult<&str, crate::AstNode> {
    let (input, name) = alpha1(name)?;

    Ok((input, AstNode::Name(name.to_string())))
}

fn parse_variable(input: &str) -> IResult<&str, crate::AstNode> {
    let (input, name) = parse_name(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, number) = alt((parse_binary_op, parse_unary_op, parse_number))(input)?;

    Ok((
        input,
        AstNode::Variable {
            name: Box::new(name),
            expr: Box::new(number),
        },
    ))
}

fn parse_unary_op(input: &str) -> IResult<&str, crate::AstNode> {
    let (input, sign) = tag("-")(input)?;
    let (input, negative_number) = parse_number(input)?;
    Ok((
        input,
        AstNode::UnaryOp {
            operation: match sign {
                "-" => UnaryOperation::Negate,
                _ => panic!("Unsupported unary operation"),
            },
            expr: Box::new(negative_number),
        },
    ))
}

fn parse_operator(input: &str) -> IResult<&str, &str> {
    Ok(alt((
        delimited(space0, tag("+"), space0),
        delimited(space0, tag("-"), space0),
        delimited(space0, tag("*"), space0),
        delimited(space0, tag("/"), space0),
        delimited(space0, tag("^"), space0),
    ))(input)?)
}

fn parse_binary_op(input: &str) -> IResult<&str, crate::AstNode> {
    let (input, lhs) = alt((parse_name, parse_unary_op, parse_number))(input)?;
    let (input, operator) = parse_operator(input)?;
    let (input, rhs) = alt((parse_name, parse_unary_op, parse_number))(input)?;
    Ok((
        input,
        AstNode::BinaryOp {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            operation: match operator {
                "" => BinaryOperation::Add,
                " - " => BinaryOperation::Subtract,
                " * " => BinaryOperation::Multiply,
                " / " => BinaryOperation::Divide,
                " ^ " => BinaryOperation::Power,
                _ => panic!("Unsupported binary operation"),
            },
        },
    ))
}

fn main() -> () {
    // let mut test_file = File::open("00-sample.bc")?;
    // let mut input_file_contents = String::new();
    // test_file.read_to_string(&mut input_file_contents)?;
    // let (input, parsed) = parse(line).unwrap();
    // if !input.is_empty() {
    //     eprintln!("parsing error, input remaining {:?}", input);
    //     process::exit(1);
    // }
    // println!("{:?}", parsed);

    assert_eq!(parse_number("11e-1"), Ok(("", AstNode::Double(1.1))));
    assert_eq!(parse_number("1"), Ok(("", AstNode::Double(1.0))));
    assert_eq!(parse_number("1.1"), Ok(("", AstNode::Double(1.1))));
    assert_eq!(
        parse_number("9999999.987654"),
        Ok(("", AstNode::Double(9999999.987654)))
    );

    assert_eq!(
        parse_name("test"),
        Ok(("", AstNode::Name("test".to_string())))
    );
    assert_eq!(
        parse_name("test"),
        Ok(("", AstNode::Name("test".to_string())))
    );

    assert_eq!(
        parse_variable("test = 1.2"),
        Ok((
            "",
            AstNode::Variable {
                name: Box::new(AstNode::Name("test".to_string())),
                expr: Box::new(AstNode::Double(1.2))
            }
        ))
    );

    assert_eq!(
        parse_unary_op("-2"),
        Ok((
            "",
            AstNode::UnaryOp {
                operation: UnaryOperation::Negate,
                expr: Box::new(AstNode::Double(2.0))
            }
        ))
    );

    assert_eq!(
        parse_binary_op("2 * 2"),
        Ok((
            "",
            AstNode::BinaryOp {
                operation: BinaryOperation::Multiply,
                lhs: Box::new(AstNode::Double(2.0)),
                rhs: Box::new(AstNode::Double(2.0))
            }
        ))
    );

    assert_eq!(
        parse_binary_op("var = -2"),
        Ok((
            "",
            AstNode::Variable {
                name: Box::new(AstNode::Name("var".to_string())),
                expr: Box::new(AstNode::UnaryOp {
                    operation: UnaryOperation::Negate,
                    expr: Box::new(AstNode::Double(2.0))
                })
            }
        ))
    );

    assert_eq!(
        parse_binary_op("var = 2 * 2"),
        Ok((
            "",
            AstNode::Variable {
                name: Box::new(AstNode::Name("var".to_string())),
                expr: Box::new(AstNode::BinaryOp {
                    operation: BinaryOperation::Multiply,
                    lhs: Box::new(AstNode::Double(2.0)),
                    rhs: Box::new(AstNode::Double(2.0))
                })
            }
        ))
    );

    ()
}
