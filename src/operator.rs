use std::str::FromStr;

#[derive(Debug)]
pub enum Op {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
}

pub fn cmp<T: PartialOrd + PartialEq>(l: T, r: T, op: &Op) -> bool {
    match op {
        Op::Eq => l == r,
        Op::Ne => l != r,
        Op::Gt => l > r,
        Op::Lt => l < r,
        Op::Ge => l >= r,
        Op::Le => l <= r,
    }
}

impl FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "==" => Ok(Op::Eq),
            "!=" => Ok(Op::Ne),
            ">" => Ok(Op::Gt),
            "<" => Ok(Op::Lt),
            ">=" => Ok(Op::Ge),
            "<=" => Ok(Op::Le),
            _ => Err(format!("Unknown operator: {s}")),
        }
    }
}
