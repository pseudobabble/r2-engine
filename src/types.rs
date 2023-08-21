use std::ops::{Add, Div, Mul, Sub};

use uom::si::f64::Length;
use uom::si::length::{kilometer, meter};

// unit calculations with exponents here
// add + sub can only work on same types
// x/y converts to x * y^-1
// add the exponents and count the terms
// x^2 * y^1 = z^3
// count the terms of the same dimension

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Unit {
    Meter,
    Kilometer,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Dimension {
    Length { unit: Unit, power: i64 },
}

impl Add for Dimension {
    type Output = Dimension;

    fn add(self, rhs: Self) -> Self {
        match self {
            // normalise to base units, meter in this case
            Dimension::Length { unit, power } => match rhs {
                Dimension::Length { unit, power } => Dimension::Length {
                    unit: Unit::Meter,
                    power: 1,
                },
            },
            _ => panic!("Cannot add {:#?} to {:#?}", self, rhs),
        }
    }
}

impl Sub for Dimension {
    type Output = Dimension;

    fn sub(self, rhs: Self) -> Self {
        match self {
            // normalise to base units, meter in this case
            // TODO zeros are probably dimensionless
            Dimension::Length { unit, power } => match rhs {
                Dimension::Length { unit, power } => Dimension::Length {
                    unit: Unit::Meter,
                    power: 1,
                },
            },
        }
    }
}

impl Mul for Dimension {
    type Output = Dimension;

    fn mul(self, rhs: Self) -> Self {
        match self {
            // normalise to base units, meter in this case
            Dimension::Length { unit, power } => match rhs {
                Dimension::Length { unit, power } => Dimension::Length {
                    unit: Unit::Meter,
                    power: power + power, // TODO: does this work?
                },
            },
        }
    }
}

impl Div for Dimension {
    type Output = Dimension;

    fn div(self, rhs: Self) -> Self {
        match self {
            // normalise to base units, meter in this case
            Dimension::Length { unit, power } => match rhs {
                Dimension::Length { unit, power } => Dimension::Length {
                    unit: Unit::Meter,
                    power: power - power,
                },
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct DimensionedValue {
    pub value: f64,
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

        let dimension = self.dimension + rhs.dimension;
        let value = (self.value + self) + rhs.value;
        println!("\nResult = {:#?}[{:#?}]", value, dimension);

        DimensionedValue {
            value: value,
            dimension: dimension, // TODO: should be the resulting values units!
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

        let dimension = self.dimension - rhs.dimension;
        let value = self.value - rhs.value;
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
            "\n\nMultiplying {:#?}[{:#?}] from {:#?}[{:#?}]",
            self.value.clone(),
            self.dimension.clone(),
            rhs.value.clone(),
            rhs.dimension.clone(),
        );

        let dimension = self.dimension * rhs.dimension;
        let value = self.value * rhs.value;
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
            "\n\nDividing {:#?}[{:#?}] from {:#?}[{:#?}]",
            self.value.clone(),
            self.dimension.clone(),
            rhs.value.clone(),
            rhs.dimension.clone(),
        );

        let dimension = self.dimension / rhs.dimension;
        let value = self.value / rhs.value;
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
