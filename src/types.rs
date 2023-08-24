use std::ops::{Add, Div, Mul, Sub};

// unit calculations with exponents here
// add + sub can only work on same types
// x/y converts to x * y^-1
// add the exponents and count the terms
// x^2 * y^1 = z^3
// count the terms of the same dimension

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(PartialEq, PartialOrd, Eq, Debug, Clone, Copy)]
pub enum UnitIdentity {
    None,
    Second,
    Minute,
    Hour,
    Day,
    Meter,
    Kilometer,
    SquareMeter,
    SquareKilometer,
    CubicMeter,
    CubicKilometer,
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Unit {
    pub unit: UnitIdentity,
    pub conversion_factor: f64,
}

impl Mul for Unit {
    type Output = Unit;

    /// 1[m^1] * 1[m^2] = 1[m^3]
    /// a^1 * a^2 = a^3
    fn mul(self, rhs: Self) -> Self {
        match self {
            UnitIdentity::Meter => match rhs {
                UnitIdentity::Meter => UnitIdentity::SquareMeter,
                UnitIdentity::SquareMeter => UnitIdentity::CubicMeter,
                _ => panic!("Cannot add {} to {}", self, rhs),
            },
            UnitIdentity::SquareMeter => match rhs {
                UnitIdentity::SquareMeter => UnitIdentity::SquareMeter,
                _ => panic!("Cannot add {} to {}", self, rhs),
            },
            UnitIdentity::CubicMeter => match rhs {
                UnitIdentity::SquareMeter => UnitIdentity::CubicMeter,
                _ => panic!("Cannot add {} to {}", self, rhs),
            },
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Dimension {
    pub unit: Unit,
    pub power: i64,
}

impl Add for Dimension {
    type Output = Dimension;

    fn add(self, rhs: Self) -> Self {
        // if LHS is Length (unit^1)
        match self.power.clone() {
            // and RHS is Length (unit^1)
            1 => match rhs.power.clone() {
                // return Length in base units
                1 => Dimension {
                    unit: Unit {
                        unit: UnitIdentity::Meter,
                        conversion_factor: 1.0,
                    },
                    power: 1,
                },
                // Cannot add Length to eg, Area
                _ => panic!(
                    "Cannot add dimensions with different powers: {:#?} to {:#?}",
                    self, rhs
                ),
            },
            2 => match rhs.power.clone() {
                2 => Dimension {
                    unit: Unit {
                        unit: UnitIdentity::SquareMeter,
                        conversion_factor: 1.0,
                    },
                    power: 2,
                },
                _ => panic!(
                    "Cannot add dimensions with different powers: {:#?} to {:#?}",
                    self, rhs
                ),
            },
            3 => match rhs.power.clone() {
                3 => Dimension {
                    unit: Unit {
                        unit: UnitIdentity::CubicMeter,
                        conversion_factor: 1.0,
                    },
                    power: 2,
                },
                _ => panic!(
                    "Cannot add dimensions with different powers: {:#?} to {:#?}",
                    self, rhs
                ),
            },
            _ => panic!("Unsupported dimension with power {}", self.power.clone()),
        }
    }
}

impl Sub for Dimension {
    type Output = Dimension;

    fn sub(self, rhs: Self) -> Self {
        match self.power.clone() {
            1 => match rhs.power.clone() {
                1 => Dimension {
                    unit: Unit {
                        unit: UnitIdentity::Meter,
                        conversion_factor: 1.0,
                    },
                    power: 1,
                },
                _ => panic!("Cannot subtract {:#?} to {:#?}", self, rhs),
            },
            2 => match rhs.power.clone() {
                2 => Dimension {
                    unit: Unit {
                        unit: UnitIdentity::SquareMeter,
                        conversion_factor: 1.0,
                    },
                    power: 2,
                },
                _ => panic!("Cannot subtract {:#?} to {:#?}", self, rhs),
            },
            3 => match rhs.power.clone() {
                3 => Dimension {
                    unit: Unit {
                        unit: UnitIdentity::CubicMeter,
                        conversion_factor: 1.0,
                    },
                    power: 3,
                },
                _ => panic!("Cannot subtract {:#?} to {:#?}", self, rhs),
            },
            _ => panic!("Unsupported dimension with power {}", self.power.clone()),
        }
    }
}

impl Mul for Dimension {
    type Output = Dimension;

    /// 1[m^1] * 1[m^2] = 1[m^3]
    /// a^1 * a^2 = a^3
    fn mul(self, rhs: Self) -> Self {
        let result_power = self.power + rhs.power;

        // this can go on impl Mul for Unit so we can just say derived_unit = self.unit * rhs.unit
        let derived_unit = match self.unit.unit.clone() {
            UnitIdentity::Meter => match rhs.unit.unit.clone() {
                UnitIdentity::Meter => UnitIdentity::SquareMeter,
                _ => panic!("Cannot add {} to {}", self.unit.unit, rhs.unit.unit),
            },
            UnitIdentity::SquareMeter => match rhs.unit.unit.clone() {
                UnitIdentity::SquareMeter => UnitIdentity::SquareMeter,
                _ => panic!("Cannot add {} to {}", self.unit.unit, rhs.unit.unit),
            },
            UnitIdentity::CubicMeter => match rhs.unit.unit.clone() {
                UnitIdentity::SquareMeter => UnitIdentity::CubicMeter,
                _ => panic!("Cannot add {} to {}", self.unit.unit, rhs.unit.unit),
            },
        }

        match result_power {
            1 => Dimension {
                unit: Unit {
                    unit: UnitIdentity::Meter,
                    conversion_factor: 1.0,
                },
                power: 1,
            },
            2 => Dimension {
                unit: Unit {
                    unit: UnitIdentity::SquareMeter,
                    conversion_factor: 1.0,
                },
                power: 2,
            },
            3 => Dimension {
                unit: Unit {
                    unit: UnitIdentity::CubicMeter,
                    conversion_factor: 1.0,
                },
                power: 3,
            },
            _ => panic!("Unsupported dimension with power {}", result_power),
        }
    }
}

impl Div for Dimension {
    type Output = Dimension;

    fn div(self, rhs: Self) -> Self {
        // 1[m^6] / 1[m^3] = 1[m^3]
        // a^6 / a^3 = a^3
        let result_power = self.power - rhs.power;

        match result_power {
            0 => Dimension {
                unit: Unit {
                    unit: UnitIdentity::None,
                    conversion_factor: 1.0,
                },
                power: 0,
            },
            1 => Dimension {
                unit: Unit {
                    unit: UnitIdentity::Meter,
                    conversion_factor: 1.0,
                },
                power: 1,
            },
            2 => Dimension {
                unit: Unit {
                    unit: UnitIdentity::SquareMeter,
                    conversion_factor: 1.0,
                },
                power: 2,
            },
            3 => Dimension {
                unit: Unit {
                    unit: UnitIdentity::CubicMeter,
                    conversion_factor: 1.0,
                },
                power: 3,
            },
            _ => panic!("Unsupported dimension with power {}", result_power),
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
