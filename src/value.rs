#[derive(Clone)]
pub struct Value {
    pub number: f64,
    pub dimension: Option<Dimension>,
}

#[derive(Clone)]
pub struct Dimension {
    // TODO
    dimension: String,
}

impl Dimension {
    pub fn new(lexeme: String) -> Self {
        Self { dimension: lexeme }
    }

    pub fn check(&self, other: Option<&Dimension>) -> bool {
        // TODO check dimensions
        true
    }

    pub fn mul_dim(&mut self, other: Option<&Dimension>) -> Self {
        // TODO multiply dimensions
        todo!()
    }

    pub fn div_dim(&mut self, other: Option<&Dimension>) -> Self {
        // TODO divide dimensions
        todo!()
    }

    pub fn pow_dim(&mut self, power: f64) -> Self {
        // TODO raise self to the power if int
        todo!()
    }
}
