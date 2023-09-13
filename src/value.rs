use std::collections::HashSet;

#[derive(Clone)]
pub struct Value {
    pub number: f64,
    pub dimension: Option<Dimension>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Unit {
    name: String,
    exponent: i64,
}

impl Unit {
    fn to_si(&self) -> Vec<Unit> {
        let res = match self.name.as_str() {
            "m" | "s" | "kg" | "A" | "K" | "mol" | "cd" => vec![Unit {
                name: self.name.clone(),
                exponent: 1,
            }],
            "Hz" => vec![Unit {
                name: "s".into(),
                exponent: -1,
            }],
            _ => vec![],
        };
        res.into_iter()
            .map(|u| Unit {
                name: u.name,
                exponent: u.exponent * self.exponent,
            })
            .collect()
    }

    fn get_lexeme(&self) -> String {
        if self.exponent == 1 {
            self.name.clone()
        } else {
            String::from(format!("{}^{}", self.name, self.exponent))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dimension {
    lexeme: String,
    units: Vec<Unit>,
}

impl Dimension {
    pub fn new(lexeme: String) -> Self {
        let units = vec![Unit {
            name: lexeme.clone(),
            exponent: 1,
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
        res
    }

    fn to_lexeme(units: Vec<Unit>) -> String {
        units
            .iter()
            .map(Unit::get_lexeme)
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn check(&self, other: &Dimension) -> bool {
        let base_left: Vec<Unit> = self.units.iter().flat_map(Unit::to_si).collect();
        let base_right: Vec<Unit> = other.units.iter().flat_map(Unit::to_si).collect();

        let base_left: HashSet<Unit> = Self::fold_units(base_left)
            .into_iter()
            .filter(|u| u.exponent != 0)
            .collect();
        let base_right: HashSet<Unit> = Self::fold_units(base_right)
            .into_iter()
            .filter(|u| u.exponent != 0)
            .collect();

        dbg!(&base_left);
        dbg!(&base_right);

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

        dbg!(&lexeme);
        dbg!(&units);
        Self { lexeme, units }
    }

    pub fn div_dim(&self, other: &Dimension) -> Self {
        let units = self
            .units
            .iter()
            .cloned()
            .chain(other.units.iter().cloned().map(|u| Unit {
                name: u.name,
                exponent: -1 * u.exponent,
            }))
            .collect();
        let units = Self::fold_units(units);
        let lexeme = Self::to_lexeme(units.clone());

        dbg!(&lexeme);
        dbg!(&units);
        Self { lexeme, units }
    }

    pub fn pow_dim(&self, power: f64) -> Self {
        // TODO raise self to the power if int
        todo!()
    }
}
