extern crate nom;
extern crate uom;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, space0};
use nom::multi::many0;
use nom::number::complete::double;
use nom::sequence::{preceded, terminated};
use nom::IResult;

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

fn parse_number(number: &str) -> IResult<&str, crate::AstNode> {
    println!("reached parse_number {}", number.clone());
    let (input, number) = double(number)?;

    Ok((input, AstNode::Double(number)))
}

fn parse_name(name: &str) -> IResult<&str, crate::AstNode> {
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

fn parse_expression(input: &str) -> IResult<&str, crate::AstNode> {
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

fn parse_variable(input: &str) -> IResult<&str, crate::AstNode> {
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

fn parse_program(input: &str) -> IResult<&str, Vec<crate::AstNode>> {
    many0(preceded(space0, parse_variable))(input)
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

    println!("\n\nTEST parse_number");
    assert_eq!(parse_number("11e-1"), Ok(("", AstNode::Double(1.1))));
    assert_eq!(parse_number("1"), Ok(("", AstNode::Double(1.0))));
    assert_eq!(parse_number("1.1"), Ok(("", AstNode::Double(1.1))));
    assert_eq!(
        parse_number("9999999.987654"),
        Ok(("", AstNode::Double(9999999.987654)))
    );

    println!("\n\nTEST parse_name");
    assert_eq!(
        parse_name("test"),
        Ok(("", AstNode::Name("test".to_string())))
    );
    assert_eq!(
        parse_name("test"),
        Ok(("", AstNode::Name("test".to_string())))
    );

    println!("\n\nTEST parse_variable double");
    assert_eq!(
        parse_variable("test = 1.2;"),
        Ok((
            "",
            AstNode::Variable {
                name: Box::new(AstNode::Name("test".to_string())),
                expr: Box::new(AstNode::Double(1.2))
            }
        ))
    );

    println!("\n\nTEST parse_variable negative unary op");
    assert_eq!(
        parse_variable("var = -2;"),
        Ok((
            "",
            AstNode::Variable {
                name: Box::new(AstNode::Name("var".to_string())),
                expr: Box::new(AstNode::Double(-2.0))
            }
        ))
    );

    println!("\n\nTEST parse_expression");
    assert_eq!(
        parse_expression("(2 / 2)"),
        Ok((
            "",
            AstNode::Expression {
                operation: BinaryOperation::Divide,
                lhs: Box::new(AstNode::Double(2.0)),
                rhs: Box::new(AstNode::Double(2.0))
            }
        ))
    );

    println!("\n\nTEST parse multi term expression");
    assert_eq!(
        parse_expression("((2 / 2) + (4 * 4))"),
        Ok((
            "",
            AstNode::Expression {
                operation: BinaryOperation::Add,
                lhs: Box::new(AstNode::Expression {
                    operation: BinaryOperation::Divide,
                    lhs: Box::new(AstNode::Double(2.0)),
                    rhs: Box::new(AstNode::Double(2.0))
                }),
                rhs: Box::new(AstNode::Expression {
                    operation: BinaryOperation::Multiply,
                    lhs: Box::new(AstNode::Double(4.0)),
                    rhs: Box::new(AstNode::Double(4.0))
                })
            }
        ))
    );

    println!("\n\nTEST parse_variable expression");
    assert_eq!(
        parse_variable("var = (2 / 2);"),
        Ok((
            "",
            AstNode::Variable {
                name: Box::new(AstNode::Name("var".to_string())),
                expr: Box::new(AstNode::Expression {
                    operation: BinaryOperation::Divide,
                    lhs: Box::new(AstNode::Double(2.0)),
                    rhs: Box::new(AstNode::Double(2.0))
                })
            }
        ))
    );

    println!("\n\nTEST parse multi term variable");
    assert_eq!(
        parse_variable("var = ((2 * 3) * (4 + 5));"),
        Ok((
            "",
            AstNode::Variable {
                name: Box::new(AstNode::Name("var".to_string())),
                expr: Box::new(AstNode::Expression {
                    operation: BinaryOperation::Multiply,
                    lhs: Box::new(AstNode::Expression {
                        operation: BinaryOperation::Multiply,
                        lhs: Box::new(AstNode::Double(2.0)),
                        rhs: Box::new(AstNode::Double(3.0)),
                    }),
                    rhs: Box::new(AstNode::Expression {
                        operation: BinaryOperation::Add,
                        lhs: Box::new(AstNode::Double(4.0)),
                        rhs: Box::new(AstNode::Double(5.0)),
                    })
                })
            }
        ))
    );

    println!("\n\nTEST parse variables and abstract expressions");
    assert_eq!(
        parse_program("x = (2 * 2); y = 1; z = (x + y);"),
        Ok((
            "",
            vec![
                AstNode::Variable {
                    name: Box::new(AstNode::Name("x".to_string())),
                    expr: Box::new(AstNode::Expression {
                        operation: BinaryOperation::Multiply,
                        lhs: Box::new(AstNode::Double(2.0)),
                        rhs: Box::new(AstNode::Double(2.0))
                    })
                },
                AstNode::Variable {
                    name: Box::new(AstNode::Name("y".to_string())),
                    expr: Box::new(AstNode::Double(1.0))
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

    println!(
        "{:#?}",
        parse_program("x = (2 * 2); y = 1; z = ((x + y) * 3);")
    );
}
