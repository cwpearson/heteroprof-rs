#[derive(PartialEq, Eq)]
pub struct Value {
    pos: u64,
    size: u64,
    initialized: bool,
}

impl Value {
    pub fn new(pos: u64, size: u64, initialized: bool) -> Value {
        let v = Value {
            pos: pos,
            size: size,
            initialized: initialized,
        };
        v
    }
}

#[derive(PartialEq, Eq)]
pub struct Values {
    latest: Value,
}

impl Values {
    pub fn new_value(pos: u64, size: u64, initialized: bool) -> Values {
        Values {
            latest: Value::new(pos, size, initialized),
        }
    }

    pub fn push(&mut self, value: Value) {
        self.latest = Value::from_value(&self.latest)
    }

    pub fn get_containing(&self, pos: u64, size: u64) -> &Value {
        return &self.latest;
    }
}
