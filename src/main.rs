use std::any::Any;

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

struct Value {
    name: str,
}

peg::parser! {
    grammar route2_parser() for str {

        rule number -> f64
            =n:$(['.'|'0'..='9']+) { n.parse::<f64>().unwrap() }

        rule variable -> Value
            = n:$(['a'..='z'|'A'..='Z'|'0'..='9'|'-']+) { n.into() }

        rule whitepace
            = [' '|'\n']*

        pub rule arithmetic() -> f64 = precedence!{
            x:(@) "+" y:@ { x + y }
            x:(@) "-" y:@ { x - y }
            --
            x:(@) "*" y:@ { x * y }
            x:(@) "/" y:@ { x / y }
            --
            x:@ "^" y:(@) { x.pow(y as u32) }
            --
            n:number() { n }
            "(" e:arithmetic() ")" { e }
        }

  }
}

fn main() {
    let x = 3.0 + 2;
    println!("Hello, world!");
}
