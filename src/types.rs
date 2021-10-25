use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Multiply,
    Div,
    Subtract,
}

impl Operator {
    pub fn apply(&self, value1: Number, value2: Number) -> Number {
        match self {
            Self::Add => value1 + value2,
            Self::Multiply => value1 * value2,
            Self::Subtract => value1 - value2,
            Self::Div => value1 / value2,
        }
    }

    pub fn precedence(&self) -> u8 {
        match self {
            Self::Add => 1,
            Self::Multiply => 2,
            Self::Subtract => 1,
            Self::Div => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(i32),
    Float(f32),
}

impl std::ops::Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(f), Self::Integer(i)) | (Self::Integer(i), Self::Float(f)) => {
                Self::Float(f + i as f32)
            }
            (Self::Integer(i1), Self::Integer(i2)) => Self::Integer(i1 + i2),
            (Self::Float(i1), Self::Float(i2)) => Self::Float(i1 + i2),
        }
    }
}

impl std::ops::Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(f), Self::Integer(i)) | (Self::Integer(i), Self::Float(f)) => {
                Number::Float(f * i as f32)
            }
            (Self::Integer(i1), Self::Integer(i2)) => Self::Integer(i1 * i2),
            (Self::Float(i1), Self::Float(i2)) => Self::Float(i1 * i2),
        }
    }
}

impl std::ops::Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(f), Self::Integer(i)) | (Self::Integer(i), Self::Float(f)) => {
                Number::Float(f - i as f32)
            }
            (Self::Integer(i1), Self::Integer(i2)) => Self::Integer(i1 - i2),
            (Self::Float(i1), Self::Float(i2)) => Self::Float(i1 - i2),
        }
    }
}

impl std::ops::Div for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(f), Self::Integer(i)) | (Self::Integer(i), Self::Float(f)) => {
                Number::Float(f / i as f32)
            }
            (Self::Integer(i1), Self::Integer(i2)) => Self::Integer(i1 / i2),
            (Self::Float(i1), Self::Float(i2)) => Self::Float(i1 / i2),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float(v) => write!(f, "{}", v.to_string()),
            Self::Integer(v) => write!(f, "{}", v.to_string()),
        }
    }
}
