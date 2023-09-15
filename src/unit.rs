#[derive(Debug, Clone)]
pub struct Unit {
    pub name: String,
    pub exponent: f64,
}

pub fn float_eq(a: f64, b: f64) -> bool {
    let d = 10f64.powi(-5);
    (a - b).abs() < d
}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && float_eq(self.exponent, other.exponent)
    }
}

impl Unit {
    pub fn to_si(&self) -> Vec<Unit> {
        let res = match self.name.as_str() {
            "m" | "s" | "kg" | "A" | "K" | "mol" | "cd" => vec![(self.name.clone(), 1.0)],
            "N" => vec![("kg".into(), 1.0), ("m".into(), 1.0), ("s".into(), -2.0)],
            "J" => vec![("kg".into(), 1.0), ("m".into(), 2.0), ("s".into(), -2.0)],
            "W" => vec![("kg".into(), 1.0), ("m".into(), 2.0), ("s".into(), -3.0)],
            "Pa" => vec![("kg".into(), 1.0), ("m".into(), -1.0), ("s".into(), -2.0)],
            "C" => vec![("A".into(), 1.0), ("s".into(), 1.0)],
            "V" => vec![
                ("kg".into(), 1.0),
                ("m".into(), 2.0),
                ("s".into(), -3.0),
                ("A".into(), -1.0),
            ],
            "F" => vec![
                ("s".into(), 4.0),
                ("A".into(), 2.0),
                ("m".into(), -2.0),
                ("kg".into(), -1.0),
            ],
            "ohm" => vec![
                ("kg".into(), 1.0),
                ("m".into(), 2.0),
                ("s".into(), -3.0),
                ("A".into(), -2.0),
            ],
            "H" => vec![
                ("kg".into(), 1.0),
                ("m".into(), 2.0),
                ("s".into(), -2.0),
                ("A".into(), -2.0),
            ],
            "Hz" => vec![("s".into(), -1.0)],
            "Bq" => vec![("s".into(), -1.0)],
            "T" => vec![("kg".into(), 1.0), ("A".into(), -1.0), ("s".into(), -2.0)],
            "Wb" => vec![
                ("kg".into(), 1.0),
                ("m".into(), 2.0),
                ("s".into(), -2.0),
                ("A".into(), -1.0),
            ],
            _ => vec![(self.name.clone(), 1.0)],
        };
        res.into_iter()
            .map(|(name, exp)| Unit {
                name,
                exponent: self.exponent * exp,
            })
            .collect()
    }

    pub fn get_lexeme(&self) -> String {
        if float_eq(self.exponent, 1.0) {
            self.name.clone()
        } else {
            format!("{}^{}", self.name, self.exponent)
        }
    }
}
