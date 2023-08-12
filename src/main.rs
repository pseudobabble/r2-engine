use std::any::Any;
use std::ops::Add;
use std::rc::Rc;

extern crate uom;

use uom::si::f32::*;
use uom::si::length::kilometer;
use uom::si::time::second;

/// (average_wage_per_unit_calendar_time * (avoided_days_of_lost_due_to_anxiety + avoided_days_of_lost_due_to_depression))
///  * ((proportion_proxy_benefit_attributed_to_wellbeing_anxiety + proportion_proxy_benefit_attributed_to_wellbeing_depression) / 2)
///  * imp_employee_time_in_mental_wellbeing_programmes

/// (average_wage_per_unit_calendar_time[currency] * (avoided_days_of_lost_due_to_anxiety[time] + avoided_days_of_lost_due_to_depression[time]))
///  * ((proportion_proxy_benefit_attributed_to_wellbeing_anxiety[float] + proportion_proxy_benefit_attributed_to_wellbeing_depression[float]) / 2)
///  * imp_employee_time_in_mental_wellbeing_programmes[time]

/// average_wage[float] = total_wage[float] / number_of_employees[int]
/// average_wage_per_unit_calendar_time =

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Identity,
}

#[derive(Debug)]
pub struct Variable {
    name: String,
    value: f32,
}

impl Variable {
    fn to_expression(self) -> Expression<Variable> {
        Expression {
            lhs: None,
            rhs: self,
            operator: Operator::Identity,
        }
    }
}

impl Add<Variable> for Variable {
    type Output = Expression<Variable>;

    fn add(self, _rhs: Variable) -> Expression<Variable> {
        Expression {
            lhs: Some(self),
            rhs: _rhs,
            operator: Operator::Add,
        }
    }
}

impl Add<Expression<Variable>> for Variable {
    type Output = Expression<Variable>;

    fn add(self, _rhs: Expression<Variable>) -> Expression<Variable> {
        Expression {
            lhs: Some(Variable {
                name: "_".to_string(),
                value: self.value + _rhs.rhs.value,
            }),
            rhs: _rhs.rhs,
            operator: Operator::Add,
        }
    }
}

#[derive(Debug)]
pub struct Expression<T> {
    lhs: Option<T>,
    rhs: T,
    operator: Operator,
}

impl Add for Expression<Variable> {
    type Output = Expression<Variable>;

    fn add(self, _rhs: Expression<Variable>) -> Expression<Variable> {
        let rhs_lhs_value = match _rhs.lhs {
            Some(value) => value.value,
            None => 0.0,
        };
        Expression {
            lhs: Some(Variable {
                name: "_".to_string(),
                value: self.rhs.value + rhs_lhs_value,
            }),
            rhs: _rhs.rhs,
            operator: Operator::Add,
        }
    }
}

peg::parser! {

    grammar route2_parser() for str {

        rule number() -> f32
            = n:$(['.'|'0'..='9']+) { n.parse::<f32>().unwrap() }

        // -1 etc
        // rule unary(num: rule<str>) -> f32
        //     = n:$(['-'num()]) { n.parse::<f32>().unwrap() * -1.0 }

        rule label() -> String
            = v:$(['a'..='z'|'A'..='Z'|'0'..='9'|'_']*) { v.into() }

        rule variable() -> Variable
            = v:label() _ "=" _ n:number() {
                Variable {
                    name: v,
                    value: n
                }
            }

        pub rule variables() -> Vec<Variable>
            = v:variable() ++ "; " { v }

        rule expression() -> Expression<Variable>
            = "(" _ e:expression() _ ")" { e }
        // whitespace
        rule _
            = [' '|'\n']*

        // pub rule parse() -> Vec<Variable> = precedence! {
        //     x:(@) _ "+" _ y:@ { x + y }
        //     // x:(@) _ "-" _ y:@ { x - y }
        //     // --
        //     // x:(@) _ "*" _ y:@ { x * y }
        //     // x:(@) _ "/" _ y:@ { x / y }
        //     // --
        //     // x:@ "^" y:(@) { f32::pow(x, y) }
        //     // --
        //     "(" _ e:parse() _ ")" { e }
        //     --
        //     // "-" x:(@) { x * -1.0}
        //     variables:variable() ++ "; " { variables }

        // }

  }
}

fn main() {
    println!("{:#?}", route2_parser::variables("x = 3; y = 5"));
    // println!("{:#?}", route2_parser::arithmetic("24.7 + 37.0"));
    // println!("{:#?}", route2_parser::arithmetic("(24.7 + 37.0) / 2"));
    // println!(
    //     "{:#?}",
    //     route2_parser::arithmetic("((24.7 + 37.0) / 2) * -1")
    // );
}
