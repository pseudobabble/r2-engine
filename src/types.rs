use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// The f64 type is the conversion factor to base units
#[derive(PartialEq, PartialOrd, Eq, Debug, Clone, Copy)]
pub enum UnitIdentity {
    CompoundUnit {
        operation: BinaryOperation,
        lhs: Box<UnitIdentity>,
        rhs: Box<UnitIdentity>,
    },
    None(f64),
    Second(f64),
    Minute(f64), // Minute(60): x[m] * 60 == x[s]
    Hour(f64),
    Day(f64),
    Meter(f64),
    Kilometer(f64),
    SquareMeter(f64),
    SquareKilometer(f64),
    CubicMeter(f64),
    CubicKilometer(f64),
    USD(f64),
    GBP(f64),
}

// TODO: implement traits for UnitIdentity
// impl Add for UnitIdentity {
//     fn add(self, rhs: Self) -> Self {
//         match self {
//             UnitIdentity::None => match
//             UnitIdentity::Second =>
//             UnitIdentity::Minute => match rhs {},
//             UnitIdentity::Hour => match rhs {},
//             UnitIdentity::Day => match rhs {},
//             UnitIdentity::Meter => match rhs {},
//             UnitIdentity::Kilometer => match rhs {},
//             UnitIdentity::MeterSquared => match rhs {},
//             UnitIdentity::KilometerSquared => match rhs {},
//             UnitIdentity::CubicMeter => match rhs {},
//             UnitIdentity::CubicKilometer => match rhs {},
//             UnitIdentity::USD => match rhs {},
//             UnitIdentity::GBP => match rhs {},
//         }
//     }
// }

#[derive(PartialEq, PartialOrd, Eq, Debug, Clone, Copy)]
pub enum Quantity {
    CompoundQuantity {
        operation: BinaryOperation,
        lhs: Box<Quantity>,
        rhs: Box<Quantity>,
    },
    Time(i64),
    Length(i64),
    Volume(i64),
    Currency(i64),
}

impl Add for Quantity {
    type Output = Unit;

    /// we can only add when the units are identical
    /// and the addition returns the same unit:
    /// 1[m^1] + 1[m^1] == 2[m^1]
    /// 1[m^1] + 1[m^2] -> invalid!
    fn add(self, rhs: Self) -> Self {
        // if simple, use, otherwise simplify
        let lhs_derived = match self {
            Quantity::Time(power) => Time(power),
            Quantity::Length(lhs_length) => Length(power),
            Quantity::Volume(lhs_length) => Volume(power),
            Quantity::Currency(lhs_length) => Currency(power),
            Quantity::CompoundQuantity {
                operation,
                lhs,
                rhs,
            } => match operation {
                BinaryOperation::Add => lhs + rhs,
                BinaryOperation::Subtract => lhs - rhs,
                BinaryOperation::Multiply => lhs * rhs, // lhs, rhs of compound lhs
                BinaryOperation::Divide => lhs / rhs,
            },
        };

        let rhs_derived = match rhs {
            Quantity::Time(power) => Time(power),
            Quantity::Length(lhs_length) => Length(power),
            Quantity::Volume(lhs_length) => Volume(power),
            Quantity::Currency(lhs_length) => Currency(power),
            Quantity::CompoundQuantity {
                lhs,
                rhs,
                operation,
            } => match operation {
                BinaryOperation::Add => lhs + rhs,
                BinaryOperation::Subtract => lhs - rhs,
                BinaryOperation::Multiply => lhs * rhs, // ?
                BinaryOperation::Divide => lhs / rhs,
            },
        };

        // if simple and type match, add powers
        // otherwise compound
        match lhs_derived {
            Quantity::Time(lhs_power) => match rhs_derived {
                // if lhs_derived == rhs_derived
                // note using lhs power in type comparison to check type and power at same time
                Quantity::Time(lhs_power) => Quantity::Time(lhs_power),
                _ => panic!("Cannot add {:#?} with {:#?}", self, rhs),
            },
            Quantity::Length(lhs_power) => match rhs_derived {
                Quantity::Length(lhs_power) => Quantity::Length(lhs_power),
                _ => panic!("Cannot add {:#?} with {:#?}", self, rhs),
            },
            Quantity::Area(lhs_power) => match rhs_derived {
                Quantity::Area(lhs_power) => Quantity::Area(lhs_power),
                _ => panic!("Cannot add {:#?} with {:#?}", self, rhs),
            },
            Quantity::Volume(lhs_power) => match rhs_derived {
                Quantity::Volume(lhs_power) => Quantity::Volume(lhs_power),
                _ => panic!("Cannot add {:#?} with {:#?}", self, rhs),
            },
            Quantity::Currency(lhs_power) => match rhs_derived {
                Quantity::Currency(lhs_power) => Quantity::Currency(lhs_power),
                _ => panic!("Cannot add {:#?} with {:#?}", self, rhs),
            },
            _ => panic!("Cannot add {:#?} with {:#?}", self, rhs),
        }
    }
}

impl Sub for Quantity {
    type Output = Unit;

    /// we can only subtract when the units are identical
    /// and the subtraction returns the same unit:
    /// 1[m^1] - 1[m^1] == 0[m^1]
    /// 1[m^1] - 1[m^2] -> invalid!
    fn sub(self, rhs: Self) -> Self {
        // if simple, use, otherwise simplify
        let lhs_derived = match self {
            Quantity::Time(power) => Quantity::Time(power),
            Quantity::Length(lhs_length) => Quantity::Length(power),
            Quantity::Volume(lhs_length) => Quantity::Volume(power),
            Quantity::Currency(lhs_length) => Quantity::Currency(power),
            Quantity::CompoundQuantity {
                operation,
                lhs,
                rhs,
            } => match operation {
                BinaryOperation::Add => lhs + rhs,
                BinaryOperation::Subtract => lhs - rhs,
                BinaryOperation::Multiply => lhs * rhs, // lhs, rhs of compound lhs
                BinaryOperation::Divide => lhs / rhs,
            },
        };

        let rhs_derived = match rhs {
            Quantity::Time(power) => Quantity::Time(power),
            Quantity::Length(lhs_length) => Quantity::Length(power),
            Quantity::Volume(lhs_length) => Quantity::Volume(power),
            Quantity::Currency(lhs_length) => Quantity::Currency(power),
            Quantity::CompoundQuantity {
                lhs,
                rhs,
                operation,
            } => match operation {
                BinaryOperation::Add => lhs + rhs,
                BinaryOperation::Subtract => lhs - rhs,
                BinaryOperation::Multiply => lhs * rhs, // ?
                BinaryOperation::Divide => lhs / rhs,
            },
        };

        // if simple and type match, add powers
        // otherwise compound
        match lhs_derived {
            Quantity::Time(lhs_power) => match rhs_derived {
                // if lhs_derived == rhs_derived
                // note using lhs power in type comparison to check type and power at same time
                Quantity::Time(lhs_power) => Quantity::Time(lhs_power),
                _ => panic!("Cannot subtract {:#?} with {:#?}", self, rhs),
            },
            Quantity::Length(lhs_power) => match rhs_derived {
                Quantity::Length(lhs_power) => Quantity::Length(lhs_power),
                _ => panic!("Cannot subtract {:#?} with {:#?}", self, rhs),
            },
            Quantity::Area(lhs_power) => match rhs_derived {
                Quantity::Area(lhs_power) => Quantity::Area(lhs_power),
                _ => panic!("Cannot subtract {:#?} with {:#?}", self, rhs),
            },
            Quantity::Volume(lhs_power) => match rhs_derived {
                Quantity::Volume(lhs_power) => Quantity::Volume(lhs_power),
                _ => panic!("Cannot subtract {:#?} with {:#?}", self, rhs),
            },
            Quantity::Currency(lhs_power) => match rhs_derived {
                Quantity::Currency(lhs_power) => Quantity::Currency(lhs_power),
                _ => panic!("Cannot subtract {:#?} with {:#?}", self, rhs),
            },
            _ => panic!("Cannot subtract {:#?} with {:#?}", self, rhs),
        }
    }
}

impl Mul for Quantity {
    type Output = Quantity;

    /// 1[m^1] * 1[m^2] = 1[m^3]
    /// a^1 * a^2 = a^3
    /// multiplying a unit returns
    /// like avaluate
    ///
    /// I think this will work
    fn mul(self, rhs: Self) -> Self {
        // if simple, use, otherwise simplify
        let lhs_derived = match self {
            Quantity::Time(power) => Quantity::Time(power),
            Quantity::Length(lhs_length) => Quantity::Length(power),
            Quantity::Volume(lhs_length) => Quantity::Volume(power),
            Quantity::Currency(lhs_length) => Quantity::Currency(power),
            Quantity::CompoundQuantity {
                operation,
                lhs,
                rhs,
            } => match operation {
                BinaryOperation::Add => lhs + rhs,
                BinaryOperation::Subtract => lhs - rhs,
                BinaryOperation::Multiply => lhs * rhs, // lhs, rhs of compound lhs
                BinaryOperation::Divide => lhs / rhs,
            },
        };

        let rhs_derived = match rhs {
            Quantity::Time(power) => Quantity::Time(power),
            Quantity::Length(lhs_length) => Quantity::Length(power),
            Quantity::Volume(lhs_length) => Quantity::Volume(power),
            Quantity::Currency(lhs_length) => Quantity::Currency(power),
            Quantity::CompoundQuantity {
                lhs,
                rhs,
                operation,
            } => match operation {
                BinaryOperation::Add => lhs + rhs,
                BinaryOperation::Subtract => lhs - rhs,
                BinaryOperation::Multiply => lhs * rhs, // ?
                BinaryOperation::Divide => lhs / rhs,
            },
        };

        // if simple and type match, add powers
        // otherwise compound
        match lhs_derived {
            Quantity::Time(lhs_power) => match rhs_derived {
                Quantity::Time(rhs_power) => Quantity::Time(lhs_power + rhs_power),
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Multiply,
                    lhs: lhs_derived,
                    rhs: rhs_derived,
                },
            },
            Quantity::Length(lhs_power) => match rhs_derived {
                Quantity::Length(rhs_power) => Quantity::Length(lhs_power + rhs_power),
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Multiply,
                    lhs: lhs_derived,
                    rhs: rhs_derived,
                },
            },
            Quantity::Area(lhs_power) => match rhs_derived {
                Quantity::Area(rhs_power) => Quantity::Area(lhs_power + rhs_power),
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Multiply,
                    lhs: lhs_derived,
                    rhs: rhs_derived,
                },
            },
            Quantity::Volume(lhs_power) => match rhs_derived {
                Quantity::Volume(rhs_power) => Quantity::Volume(lhs_power + rhs_power),
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Multiply,
                    lhs: lhs_derived,
                    rhs: rhs_derived,
                },
            },
            Quantity::Currency(lhs_power) => match rhs_derived {
                Quantity::Currency(rhs_power) => Quantity::Currency(lhs_power + rhs_power),
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Multiply,
                    lhs: lhs_derived,
                    rhs: rhs_derived,
                },
            },
            Quantity::CompoundQuantity {
                operation,
                lhs,
                rhs,
            } => Quantity::CompoundQuantity {
                operation: BinaryOperation::Multiply,
                lhs: lhs_derived,
                rhs: rhs_derived,
            },
        }
    }
}

impl Div for Quantity {
    type Output = Quantity;

    /// 1[m^1] * 1[m^2] = 1[m^3]
    /// a^1 * a^2 = a^3
    fn div(self, rhs: Self) -> Self {
        // if simple, use, otherwise simplify
        let lhs_derived = match self {
            Quantity::Time(power) => Quantity::Time(power),
            Quantity::Length(lhs_length) => Quantity::Length(power),
            Quantity::Volume(lhs_length) => Quantity::Volume(power),
            Quantity::Currency(lhs_length) => Quantity::Currency(power),
            Quantity::CompoundQuantity {
                operation,
                lhs,
                rhs,
            } => match operation {
                BinaryOperation::Add => lhs + rhs,
                BinaryOperation::Subtract => lhs - rhs,
                BinaryOperation::Multiply => lhs * rhs, // lhs, rhs of compound lhs
                BinaryOperation::Divide => lhs / rhs,
            },
        };

        let rhs_derived = match rhs {
            Quantity::Time(power) => Quantity::Time(power),
            Quantity::Length(lhs_length) => Quantity::Length(power),
            Quantity::Volume(lhs_length) => Quantity::Volume(power),
            Quantity::Currency(lhs_length) => Quantity::Currency(power),
            Quantity::CompoundQuantity {
                lhs,
                rhs,
                operation,
            } => match operation {
                BinaryOperation::Add => lhs + rhs,
                BinaryOperation::Subtract => lhs - rhs,
                BinaryOperation::Multiply => lhs * rhs, // ?
                BinaryOperation::Divide => lhs / rhs,
            },
        };

        // if simple and type match, add powers
        // otherwise compound
        match lhs_derived {
            Quantity::Time(lhs_power) => match rhs_derived {
                Quantity::Time(rhs_power) => Quantity::Time(lhs_power + rhs_power),
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Multiply,
                    lhs: lhs_derived,
                    rhs: rhs_derived,
                },
            },
            Quantity::Length(lhs_power) => match rhs_derived {
                Quantity::Length(rhs_power) => Quantity::Length(lhs_power + rhs_power),
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Multiply,
                    lhs: lhs_derived,
                    rhs: rhs_derived,
                },
            },
            Quantity::Area(lhs_power) => match rhs_derived {
                Quantity::Area(rhs_power) => Quantity::Area(lhs_power + rhs_power),
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Multiply,
                    lhs: lhs_derived,
                    rhs: rhs_derived,
                },
            },
            Quantity::Volume(lhs_power) => match rhs_derived {
                Quantity::Volume(rhs_power) => Quantity::Volume(lhs_power + rhs_power),
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Multiply,
                    lhs: lhs_derived,
                    rhs: rhs_derived,
                },
            },
            Quantity::Currency(lhs_power) => match rhs_derived {
                Quantity::Currency(rhs_power) => Quantity::Currency(lhs_power + rhs_power),
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Multiply,
                    lhs: lhs_derived,
                    rhs: rhs_derived,
                },
            },
            Quantity::CompoundQuantity {
                operation,
                lhs,
                rhs,
            } => Quantity::CompoundQuantity {
                operation: BinaryOperation::Multiply,
                lhs: lhs_derived,
                rhs: rhs_derived,
            },
        }
    }
}

/// unit contains conversion factor
/// quantity contains power
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Unit {
    pub unit: UnitIdentity,
    pub quantity: Quantity,
}

impl Add for Unit {
    type Output = Unit;

    fn add(self, rhs: Self) -> Self {
        // if same quantity and same power, fine, otherwise panic
        if self.quantity.clone() == rhs.quantity.clone() {
            if self.power.clone() == rhs.power.clone() {
                Unit {
                    unit: self.unit,
                    quantity: self.quantity,
                }
            } else {
                panic!("Cannot add {:#?} with {:#?}", self, rhs)
            }
        } else {
            panic!("Cannot add {:#?} with {:#?}", self, rhs)
        }
    }
}

impl Sub for Unit {
    type Output = Unit;

    fn add(self, rhs: Self) -> Self {
        // if same quantity and same power, fine, otherwise panic
        if self.quantity.clone() == rhs.quantity.clone() {
            if self.power.clone() == rhs.power.clone() {
                Unit {
                    unit: self.unit,
                    quantity: self.quantity,
                }
            } else {
                panic!("Cannot add {:#?} with {:#?}", self, rhs)
            }
        } else {
            panic!("Cannot add {:#?} with {:#?}", self, rhs)
        }
    }
}

impl Mul for Unit {
    type Output = Unit;

    /// 1[m^1] * 1[m^2] = 1[m^3]
    /// a^1 * a^2 = a^3
    fn mul(self, rhs: Self) -> Self {
        Unit {
            unit: self.unit * rhs.unit,
            quantity: self.quantity * rhs.quantity,
        }
    }
}

impl Div for Unit {
    type Output = Unit;

    fn div(self, rhs: Self) -> Self {
        let result_power = self.power + rhs.power;

        Unit {
            unit: self.unit / rhs.unit,
            quantity: self.quantity / rhs.quantity,
            power: result_power,
            conversion_factor: self.conversion_factor,
        }
    }
}

// make thhe value an enum like Value::Float64, Value::VecFloat64
// then can remove all the generic types
// and how to multiply vec<64> and f64 gets pushed into impl Mul for Value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Float(f64),
    Vec(Vec<f64>),
}

fn scalar_vector_addition(scalar: f64, vector: Vec<f64>) -> Vec<f64> {
    vector.iter().map(|left_x| left_x + scalar).collect()
}

fn scalar_vector_subtraction(scalar: f64, vector: Vec<f64>) -> Vec<f64> {
    vector.iter().map(|left_x| left_x - scalar).collect()
}

fn scalar_vector_multiplication(scalar: f64, vector: Vec<f64>) -> Vec<f64> {
    vector.iter().map(|left_x| left_x * scalar).collect()
}

fn scalar_vector_division(scalar: f64, vector: Vec<f64>) -> Vec<f64> {
    vector.iter().map(|left_x| left_x / scalar).collect()
}

fn elementwise_vector_addition(lhs_value: Vec<f64>, rhs_value: Vec<f64>) -> Vec<f64> {
    lhs_value
        .iter()
        .zip(rhs_value)
        .map(|(left_x, right_x)| left_x + right_x)
        .collect()
}

fn elementwise_vector_subtraction(lhs_value: Vec<f64>, rhs_value: Vec<f64>) -> Vec<f64> {
    lhs_value
        .iter()
        .zip(rhs_value)
        .map(|(left_x, right_x)| left_x - right_x)
        .collect()
}

fn elementwise_vector_multiplication(lhs_value: Vec<f64>, rhs_value: Vec<f64>) -> Vec<f64> {
    lhs_value
        .iter()
        .zip(rhs_value)
        .map(|(left_x, right_x)| left_x * right_x)
        .collect()
}

fn elementwise_vector_division(lhs_value: Vec<f64>, rhs_value: Vec<f64>) -> Vec<f64> {
    lhs_value
        .iter()
        .zip(rhs_value)
        .map(|(left_x, right_x)| left_x / right_x)
        .collect()
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self {
        match self {
            // we are float
            Value::Float(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => Value::Float(lhs_value + rhs_value),
                // they are vec
                Value::Vec(rhs_value) => Value::Vec(scalar_vector_addition(lhs_value, rhs_value)),
            },
            // we are vec
            Value::Vec(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => Value::Vec(scalar_vector_addition(rhs_value, lhs_value)),
                // they are vec
                Value::Vec(rhs_value) => {
                    Value::Vec(elementwise_vector_addition(lhs_value, rhs_value))
                }
            },
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self {
        match self {
            // we are float
            Value::Float(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => Value::Float(lhs_value + rhs_value),
                // they are vec
                Value::Vec(rhs_value) => {
                    Value::Vec(scalar_vector_subtraction(lhs_value, rhs_value))
                }
            },
            // we are vec
            Value::Vec(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => {
                    Value::Vec(scalar_vector_subtraction(rhs_value, lhs_value))
                }
                // they are vec
                Value::Vec(rhs_value) => {
                    Value::Vec(elementwise_vector_subtraction(lhs_value, rhs_value))
                }
            },
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self {
        match self {
            // we are float
            Value::Float(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => Value::Float(lhs_value * rhs_value),
                // they are vec
                Value::Vec(rhs_value) => {
                    Value::Vec(scalar_vector_multiplication(lhs_value, rhs_value))
                }
            },
            // we are vec
            Value::Vec(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => {
                    Value::Vec(scalar_vector_multiplication(rhs_value, lhs_value))
                }
                // they are vec
                Value::Vec(rhs_value) => {
                    Value::Vec(elementwise_vector_multiplication(lhs_value, rhs_value))
                }
            },
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self {
        match self {
            // we are float
            Value::Float(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => Value::Float(lhs_value * rhs_value),
                // they are vec
                Value::Vec(rhs_value) => Value::Vec(scalar_vector_division(lhs_value, rhs_value)),
            },
            // we are vec
            Value::Vec(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => Value::Vec(scalar_vector_division(rhs_value, lhs_value)),
                // they are vec
                Value::Vec(rhs_value) => {
                    Value::Vec(elementwise_vector_division(lhs_value, rhs_value))
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct DimensionedValue {
    pub value: Value,
    pub dimension: Dimension,
}

impl Add for DimensionedValue {
    type Output = DimensionedValue;

    fn add(self, rhs: Self) -> Self {
        println!(
            "\n\nAdding {:#?}[{:#?}] to {:#?}[{:#?}]",
            self.value.clone(),
            self.dimension.clone(),
            rhs.value.clone(),
            rhs.dimension.clone(),
        );

        let lhs_value_in_base_units =
            self.value * Value::Float(self.dimension.unit.conversion_factor);
        let rhs_value_in_base_units =
            rhs.value * Value::Float(rhs.dimension.unit.conversion_factor);
        let dimension = self.dimension + rhs.dimension;
        let value = lhs_value_in_base_units + rhs_value_in_base_units;
        println!("\nResult = {:#?}[{:#?}]", value, dimension);

        DimensionedValue {
            value: value,
            dimension: dimension,
        }
    }
}

impl Sub for DimensionedValue {
    type Output = DimensionedValue;

    fn sub(self, rhs: Self) -> Self {
        println!(
            "\n\nSubtracting {:#?}[{:#?}] from {:#?}[{:#?}]",
            rhs.value.clone(),
            rhs.dimension.clone(),
            self.value.clone(),
            self.dimension.clone()
        );

        let lhs_value_in_base_units =
            self.value * Value::Float(self.dimension.unit.conversion_factor);
        let rhs_value_in_base_units =
            rhs.value * Value::Float(rhs.dimension.unit.conversion_factor);
        let dimension = self.dimension - rhs.dimension;
        let value = lhs_value_in_base_units - rhs_value_in_base_units;
        println!("\nResult = {:#?}[{:#?}]", value, dimension);

        DimensionedValue {
            value: value,
            dimension: dimension,
        }
    }
}

impl Mul for DimensionedValue {
    type Output = DimensionedValue;

    fn mul(self, rhs: Self) -> Self {
        println!(
            "\n\nMultiplying {:#?}[{:#?}] with {:#?}[{:#?}]",
            self.value.clone(),
            self.dimension.clone(),
            rhs.value.clone(),
            rhs.dimension.clone(),
        );

        let lhs_value_in_base_units =
            self.value * Value::Float(self.dimension.unit.conversion_factor);
        let rhs_value_in_base_units =
            rhs.value * Value::Float(rhs.dimension.unit.conversion_factor);
        let dimension = self.dimension * rhs.dimension;
        let value = lhs_value_in_base_units * rhs_value_in_base_units;
        println!("\nResult = {:#?}[{:#?}]", value, dimension);

        DimensionedValue {
            value: value,
            dimension: dimension,
        }
    }
}

impl Div for DimensionedValue {
    type Output = DimensionedValue;

    fn div(self, rhs: Self) -> Self {
        println!(
            "\n\nDividing {:#?}[{:#?}] into {:#?}[{:#?}]",
            self.value.clone(),
            self.dimension.clone(),
            rhs.value.clone(),
            rhs.dimension.clone(),
        );

        let lhs_value_in_base_units =
            self.value * Value::Float(self.dimension.unit.conversion_factor);
        let rhs_value_in_base_units =
            rhs.value * Value::Float(rhs.dimension.unit.conversion_factor);
        let dimension = self.dimension / rhs.dimension;
        let value = lhs_value_in_base_units / rhs_value_in_base_units;
        println!("\nResult = {:#?}[{:#?}]", value, dimension);

        DimensionedValue {
            value: value,
            dimension: dimension,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Print(Box<AstNode>),
    Double {
        value: Value,
        dimension: Dimension,
    },
    Vector {
        value: Value,
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
