use crate::unit::{float_eq, Unit};

#[derive(Clone)]
pub struct Value {
    pub number: f64,
    pub dimension: Option<Dimension>,
}

impl Value {
    pub fn is_dimensionless(&self) -> bool {
        if let Some(dim) = &self.dimension {
            dim.is_dimensionless()
        } else {
            true
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dimension {
    pub lexeme: String,
    units: Vec<Unit>,
}

impl Dimension {
    pub fn new(lexeme: String) -> Self {
        let units = vec![Unit {
            name: lexeme.clone(),
            exponent: 1.0,
        }];
        Self { lexeme, units }
    }

    fn fold_units(units: Vec<Unit>) -> Vec<Unit> {
        let res: Vec<Unit> = units.iter().fold(vec![], |mut acc, unit| {
            if let Some(entry) = acc.iter_mut().find(|u| u.name == unit.name) {
                entry.exponent += unit.exponent;
            } else {
                acc.push(unit.clone());
            }
            acc
        });
        res.into_iter().filter(|u| !float_eq(u.exponent, 0.0)).collect()
    }

    fn to_lexeme(units: Vec<Unit>) -> String {
        units
            .iter()
            .map(Unit::get_lexeme)
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn is_dimensionless(&self) -> bool {
        let si: Vec<Unit> = self.units.iter().flat_map(Unit::to_si).collect();
        let si = Self::fold_units(si);
        si.iter()
            .filter(|u| !float_eq(u.exponent, 0.0))
            .collect::<Vec<&Unit>>()
            .is_empty()
    }

    pub fn check(&self, other: Option<&Dimension>) -> bool {
        if other.is_none() {
            return self.is_dimensionless();
        }
        let base_left: Vec<Unit> = self.units.iter().flat_map(Unit::to_si).collect();
        let base_right: Vec<Unit> = other.unwrap().units.iter().flat_map(Unit::to_si).collect();

        let mut base_left: Vec<Unit> = Self::fold_units(base_left)
            .into_iter()
            .filter(|u| !float_eq(u.exponent, 0.0))
            .collect();
        let mut base_right: Vec<Unit> = Self::fold_units(base_right)
            .into_iter()
            .filter(|u| !float_eq(u.exponent, 0.0))
            .collect();

        base_left.sort_by(|a, b| a.name.cmp(&b.name));
        base_right.sort_by(|a, b| a.name.cmp(&b.name));

        base_left == base_right
    }

    pub fn mul_dim(&self, other: &Dimension) -> Self {
        let units = self
            .units
            .iter()
            .cloned()
            .chain(other.units.iter().cloned())
            .collect();
        let units = Self::fold_units(units);
        let lexeme = Self::to_lexeme(units.clone());

        Self { lexeme, units }
    }

    pub fn div_dim(&self, other: &Dimension) -> Self {
        let units = self
            .units
            .iter()
            .cloned()
            .chain(other.units.iter().cloned().map(|u| Unit {
                name: u.name,
                exponent: -1.0 * u.exponent,
            }))
            .collect();
        let units = Self::fold_units(units);
        let lexeme = Self::to_lexeme(units.clone());

        Self { lexeme, units }
    }

    pub fn pow_dim(&self, power: f64) -> Self {
        let units = self
            .units
            .iter()
            .cloned()
            .map(|u| Unit {
                name: u.name,
                exponent: u.exponent * power,
            })
            .collect();
        let units = Self::fold_units(units);
        let lexeme = Self::to_lexeme(units.clone());

        Self { lexeme, units }
    }
}
