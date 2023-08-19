extern crate uom;

use std::fs::File;
use std::io::Read;
use std::process;

pub mod interpreter;
pub mod parser;
pub mod types;

use parser::*;

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
///
///
/// TODO: do distance/speed/time examples -> need the interpreter
/// TODO: variables need units, otherwise can do valid but unintended calculations

fn main() -> () {
    println!(
        "{:#?}",
        parse_line("x = (2[m] * 2[kilometer]); y = 1[km]; z = (x + y);")
    );

    let mut test_file = File::open("./test.r2").unwrap();
    let mut input_file_contents = String::new();
    test_file.read_to_string(&mut input_file_contents).unwrap();

    let mut program = vec![];
    for line in input_file_contents.lines().by_ref() {
        if line.is_empty() {
            continue;
        }
        let (input, parsed_line) = parse_line(line).unwrap();
        if !input.is_empty() {
            eprintln!("parsing error, input remaining {:?}", input);
            process::exit(1);
        }
        program.push(parsed_line);

        println!("{:#?}", program);
        // let result = evaluate(parsed_line);
        // println!("{:?}", result);
        ()
    }
}
