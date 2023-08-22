use std::ops::{Add, Div, Mul, Sub};

use ndarray::Array;

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
                _ => panic!("Cannot add {:#?} to {:#?}", self, rhs),
            },
            2 => match rhs.power.clone() {
                2 => Dimension {
                    unit: Unit {
                        unit: UnitIdentity::SquareMeter,
                        conversion_factor: 1.0,
                    },
                    power: 2,
                },
                _ => panic!("Cannot add {:#?} to {:#?}", self, rhs),
            },
            3 => match rhs.power.clone() {
                3 => Dimension {
                    unit: Unit {
                        unit: UnitIdentity::CubicMeter,
                        conversion_factor: 1.0,
                    },
                    power: 2,
                },
                _ => panic!("Cannot add {:#?} to {:#?}", self, rhs),
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

#[derive(Debug, Clone, Copy)]
pub struct DimensionedValue<T: Clone> {
    pub value: T,
    pub dimension: Dimension,
}

impl Add for DimensionedValue<f64> {
    type Output = DimensionedValue<f64>;

    fn add(self, rhs: Self) -> Self {
        println!(
            "\n\nAdding {:#?}[{:#?}] to {:#?}[{:#?}]",
            self.value.clone(),
            self.dimension.clone(),
            rhs.value.clone(),
            rhs.dimension.clone(),
        );

        let lhs_value_in_base_units = self.value * self.dimension.unit.conversion_factor;
        let rhs_value_in_base_units = rhs.value * rhs.dimension.unit.conversion_factor;
        let dimension = self.dimension + rhs.dimension;
        let value = lhs_value_in_base_units + rhs_value_in_base_units;
        println!("\nResult = {:#?}[{:#?}]", value, dimension);

        DimensionedValue {
            value: value,
            dimension: dimension,
        }
    }
}

impl Add for DimensionedValue<Vec<f64>> {
    type Output = DimensionedValue<Vec<f64>>;

    fn add(self, rhs: Self) -> Self {
        println!(
            "\n\nAdding {:#?}[{:#?}] to {:#?}[{:#?}]",
            self.value.clone(),
            self.dimension.clone(),
            rhs.value.clone(),
            rhs.dimension.clone(),
        );

        let dimension = self.dimension + rhs.dimension;

        let lhs_value_in_base_units =
            Array::from_vec(self.value) * self.dimension.unit.conversion_factor;
        let rhs_value_in_base_units =
            Array::from_vec(rhs.value) * rhs.dimension.unit.conversion_factor;

        let value = lhs_value_in_base_units
            .to_vec()
            .iter()
            .zip(rhs_value_in_base_units)
            .map(|(left_x, right_x)| left_x + right_x)
            .collect();
        println!("\nResult = {:#?}[{:#?}]", value, dimension);

        DimensionedValue {
            value: value,
            dimension: dimension,
        }
    }
}

impl Sub for DimensionedValue<f64> {
    type Output = DimensionedValue<f64>;

    fn sub(self, rhs: Self) -> Self {
        println!(
            "\n\nSubtracting {:#?}[{:#?}] from {:#?}[{:#?}]",
            rhs.value.clone(),
            rhs.dimension.clone(),
            self.value.clone(),
            self.dimension.clone()
        );

        let lhs_value_in_base_units = self.value * self.dimension.unit.conversion_factor;
        let rhs_value_in_base_units = rhs.value * rhs.dimension.unit.conversion_factor;
        let dimension = self.dimension - rhs.dimension;
        let value = lhs_value_in_base_units - rhs_value_in_base_units;
        println!("\nResult = {:#?}[{:#?}]", value, dimension);

        DimensionedValue {
            value: value,
            dimension: dimension,
        }
    }
}

impl Sub for DimensionedValue<Vec<f64>> {
    type Output = DimensionedValue<Vec<f64>>;

    fn sub(self, rhs: Self) -> Self {
        println!(
            "\n\nSubtracting {:#?}[{:#?}] from {:#?}[{:#?}]",
            rhs.value.clone(),
            rhs.dimension.clone(),
            self.value.clone(),
            self.dimension.clone()
        );

        let dimension = self.dimension + rhs.dimension;

        let lhs_value_in_base_units =
            Array::from_vec(self.value) * self.dimension.unit.conversion_factor;
        let rhs_value_in_base_units =
            Array::from_vec(rhs.value) * rhs.dimension.unit.conversion_factor;

        let value = lhs_value_in_base_units
            .to_vec()
            .iter()
            .zip(rhs_value_in_base_units)
            .map(|(left_x, right_x)| left_x - right_x)
            .collect();
        println!("\nResult = {:#?}[{:#?}]", value, dimension);

        DimensionedValue {
            value: value,
            dimension: dimension,
        }
    }
}

impl Mul for DimensionedValue<f64> {
    type Output = DimensionedValue<f64>;

    fn mul(self, rhs: Self) -> Self {
        println!(
            "\n\nMultiplying {:#?}[{:#?}] with {:#?}[{:#?}]",
            self.value.clone(),
            self.dimension.clone(),
            rhs.value.clone(),
            rhs.dimension.clone(),
        );

        let lhs_value_in_base_units = self.value * self.dimension.unit.conversion_factor;
        let rhs_value_in_base_units = rhs.value * rhs.dimension.unit.conversion_factor;
        let dimension = self.dimension * rhs.dimension;
        let value = lhs_value_in_base_units * rhs_value_in_base_units;
        println!("\nResult = {:#?}[{:#?}]", value, dimension);

        DimensionedValue {
            value: value,
            dimension: dimension,
        }
    }
}

impl Mul for DimensionedValue<Vec<f64>> {
    type Output = DimensionedValue<Vec<f64>>;

    fn mul(self, rhs: Self) -> Self {
        println!(
            "\n\nMultiplying {:#?}[{:#?}] with {:#?}[{:#?}]",
            self.value.clone(),
            self.dimension.clone(),
            rhs.value.clone(),
            rhs.dimension.clone(),
        );

        let dimension = self.dimension + rhs.dimension;

        let lhs_value_in_base_units =
            Array::from_vec(self.value) * self.dimension.unit.conversion_factor;
        let rhs_value_in_base_units =
            Array::from_vec(rhs.value) * rhs.dimension.unit.conversion_factor;

        let value = lhs_value_in_base_units
            .to_vec()
            .iter()
            .zip(rhs_value_in_base_units)
            .map(|(left_x, right_x)| left_x * right_x)
            .collect();
        println!("\nResult = {:#?}[{:#?}]", value, dimension);

        DimensionedValue {
            value: value,
            dimension: dimension,
        }
    }
}

impl Div for DimensionedValue<f64> {
    type Output = DimensionedValue<f64>;

    fn div(self, rhs: Self) -> Self {
        println!(
            "\n\nDividing {:#?}[{:#?}] into {:#?}[{:#?}]",
            self.value.clone(),
            self.dimension.clone(),
            rhs.value.clone(),
            rhs.dimension.clone(),
        );

        let lhs_value_in_base_units = self.value * self.dimension.unit.conversion_factor;
        let rhs_value_in_base_units = rhs.value * rhs.dimension.unit.conversion_factor;
        let dimension = self.dimension / rhs.dimension;
        let value = lhs_value_in_base_units / rhs_value_in_base_units;
        println!("\nResult = {:#?}[{:#?}]", value, dimension);

        DimensionedValue {
            value: value,
            dimension: dimension,
        }
    }
}

impl Div for DimensionedValue<Vec<f64>> {
    type Output = DimensionedValue<Vec<f64>>;

    fn div(self, rhs: Self) -> Self {
        println!(
            "\n\nDividing {:#?}[{:#?}] into {:#?}[{:#?}]",
            self.value.clone(),
            self.dimension.clone(),
            rhs.value.clone(),
            rhs.dimension.clone(),
        );

        let dimension = self.dimension + rhs.dimension;

        let lhs_value_in_base_units =
            Array::from_vec(self.value) * self.dimension.unit.conversion_factor;
        let rhs_value_in_base_units =
            Array::from_vec(rhs.value) * rhs.dimension.unit.conversion_factor;

        let value = lhs_value_in_base_units
            .to_vec()
            .iter()
            .zip(rhs_value_in_base_units)
            .map(|(left_x, right_x)| left_x / right_x)
            .collect();
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
        value: f64,
        dimension: Dimension,
    },
    Vector {
        value: Vec<f64>,
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
