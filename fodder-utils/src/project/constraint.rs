use super::Version;


#[derive(PartialEq, Eq, Debug)]
pub struct Constraint{
    low_v: Version,
    low_op: Op,
    high_op: Op,
    high_v: Version,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Op {
    Less,
    LessEq,
}

impl Op {
    fn verify(self, l: &Version, r: &Version) -> bool {
        if (self == Op::LessEq) & (l == r) { true } else { l < r }
    }
}

impl Constraint {
    pub fn verify(self, mid_v: &Version) -> bool {
        self.low_op.verify(&self.low_v, mid_v)
        & self.high_op.verify(mid_v, &self.high_v)
    }
}


from_str! { Constraint |vstr| {
    let items: Vec<&str> = vstr
        .split(' ')
        .collect();
    match items.as_slice() {
        [low_v, low_op, "v", high_op, high_v] => {
            Some(Constraint {
                low_v: Version::from_str(low_v)?,
                low_op: Op::from_str(low_op)?,
                high_op: Op::from_str(high_op)?,
                high_v: Version::from_str(high_v)?,
            })
        }
        _ => None,
    }
}}

to_str! { Constraint |c, f| {
    write!(f, "{} {} v {} {}",
        c.low_v, c.low_op, c.high_op, c.high_v,
    )
}}

from_str! { Op |s| {
    match s {
        "<" => Some(Op::Less),
        "<=" => Some(Op::LessEq),
        _ => None,
    }
}}

to_str! { Op |op, f| {
    write!(f, "{}", match op {
        Op::Less => "<",
        Op::LessEq => "<=",
    })
}}
