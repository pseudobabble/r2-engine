use std::ops::{Add, Div, Mul, Sub};

use uom::si::f64::Length;
use uom::si::length::{kilometer, meter};

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
    Length { unit: Unit },
}

#[derive(Debug, Clone)]
pub struct DimensionedValue {
    pub value: f64,
    pub dimension: Dimension,
}

impl Add for DimensionedValue {
    type Output = DimensionedValue;

    fn add(self, rhs: Self) -> Self {
        // unit calculation, back to base units
        let lhs_value = match self.dimension.clone() {
            Dimension::Length { unit: unit } => match unit {
                Unit::Meter => Length::new::<meter>(self.value),
                Unit::Kilometer => Length::new::<kilometer>(self.value),
            },
        };

        let rhs_value = match self.dimension.clone() {
            Dimension::Length { unit: unit } => match unit {
                Unit::Meter => Length::new::<meter>(rhs.value),
                Unit::Kilometer => Length::new::<kilometer>(rhs.value),
            },
        };

        let value = lhs_value + rhs_value;

        DimensionedValue {
            value: value.value,
            dimension: Dimension::Length { unit: Unit::Meter }, // TODO: should be the resulting values units!
        }
    }
}

impl Sub for DimensionedValue {
    type Output = DimensionedValue;

    fn sub(self, rhs: Self) -> Self {
        // unit calculation, back to base units
        let lhs_value = match self.dimension.clone() {
            Dimension::Length { unit: unit } => match unit {
                Unit::Meter => Length::new::<meter>(self.value),
                Unit::Kilometer => Length::new::<kilometer>(self.value),
            },
        };

        let rhs_value = match self.dimension.clone() {
            Dimension::Length { unit: unit } => match unit {
                Unit::Meter => Length::new::<meter>(rhs.value),
                Unit::Kilometer => Length::new::<kilometer>(rhs.value),
            },
        };

        let value = lhs_value - rhs_value;

        DimensionedValue {
            value: value.value,
            dimension: self.dimension,
        }
    }
}

impl Mul for DimensionedValue {
    type Output = DimensionedValue;

    fn mul(self, rhs: Self) -> Self {
        // unit calculation, back to base units
        let lhs_value = match self.dimension.clone() {
            Dimension::Length { unit: unit } => match unit {
                Unit::Meter => Length::new::<meter>(self.value),
                Unit::Kilometer => Length::new::<kilometer>(self.value),
            },
        };

        let rhs_value = match self.dimension.clone() {
            Dimension::Length { unit: unit } => match unit {
                Unit::Meter => Length::new::<meter>(rhs.value),
                Unit::Kilometer => Length::new::<kilometer>(rhs.value),
            },
        };

        let value = lhs_value * rhs_value;

        DimensionedValue {
            value: value.value,
            dimension: self.dimension,
        }
    }
}

impl Div for DimensionedValue {
    type Output = DimensionedValue;

    fn div(self, rhs: Self) -> Self {
        // unit calculation, back to base units
        let lhs_value = match self.dimension.clone() {
            Dimension::Length { unit: unit } => match unit {
                Unit::Meter => Length::new::<meter>(self.value),
                Unit::Kilometer => Length::new::<kilometer>(self.value),
            },
        };

        let rhs_value = match self.dimension.clone() {
            Dimension::Length { unit: unit } => match unit {
                Unit::Meter => Length::new::<meter>(rhs.value),
                Unit::Kilometer => Length::new::<kilometer>(rhs.value),
            },
        };

        let value = lhs_value / rhs_value;

        DimensionedValue {
            value: value.value,
            dimension: self.dimension,
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
