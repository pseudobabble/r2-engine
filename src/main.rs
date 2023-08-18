extern crate nom;
extern crate uom;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char, space0};
use nom::multi::many0;
use nom::number::complete::double;
use nom::sequence::{delimited, preceded, terminated};
use nom::IResult;

// TODO: dimensional analysis can happen when interpreted
// use uom::fmt::DisplayStyle::Abbreviation;
// use uom::si::f64;
// use uom::si::f64::*;
// use uom::si::length::meter;
// use uom::si::time::second;
// use uom::si::velocity::{kilometer_per_second, meter_per_second};
// use uom::si::Quantity;
// use uom::Conversion;

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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Unit {
    Meter,
    Kilometer,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Dimension {
    Length { unit: Unit },
    Volume { unit: Unit },
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Print(Box<AstNode>),
    Double {
        value: f64,
        dimension: Dimension,
    },
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

fn parse_length(input: &str) -> IResult<&str, crate::Dimension> {
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
        "m" => Dimension::Length { unit: Unit::Meter },
        "meter" => Dimension::Length { unit: Unit::Meter },
        "meters" => Dimension::Length { unit: Unit::Meter },
        "km" => Dimension::Length {
            unit: Unit::Kilometer,
        },
        "kilometer" => Dimension::Length {
            unit: Unit::Kilometer,
        },
        "kilometers" => Dimension::Length {
            unit: Unit::Kilometer,
        },
        _ => panic!("Unsupported unit alias {}", unit_alias),
    };

    Ok((input, dimension))
}

/// Switch on dimensions
fn parse_dimension(input: &str) -> IResult<&str, crate::Dimension> {
    println!("reached parse_dimension {}", input.clone());
    let (input, dimension) = parse_length(input)?;
    // let (input, dimension) = delimited(tag("["), alt((parse_length, parse_volume)), tag("]"))(input)?;

    Ok((input, dimension))
}

fn parse_number(number: &str) -> IResult<&str, crate::AstNode> {
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

    println!("\n\nTEST parse_variable negative unary op");
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

    println!("\n\nTEST parse_expression");
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

    println!("\n\nTEST parse multi term expression");
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

    println!("\n\nTEST parse_variable expression");
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

    println!("\n\nTEST parse multi term variable");
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

    println!("\n\nTEST parse variables and abstract expressions");
    assert_eq!(
        parse_program("x = (2[m] * 2[kilometer]); y = 1[km]; z = (x + y);"),
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

    println!(
        "{:#?}",
        parse_program("x = (2[m] * 2[kilometer]); y = 1[km]; z = (x + y);")
    );

    // let l1 = Length::new::<meter>(15.0);
    // let l2 = Length::new::<centimeter>(10.0);
    // let t1 = Time::new::<second>(50.0);
    // let v1 = l1 / t1;
    // // let error = l1 + t1;

    // struct TypedVar<T> {
    //     name: String,
    //     typename: T,
    // }

    // let s = TypedVar {
    //     name: "t".to_string(),
    //     typename: Length::new::<meter>,
    // };
}
