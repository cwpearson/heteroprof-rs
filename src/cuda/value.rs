#[derive(PartialEq, Eq)]
pub struct Value {
    pos: u64,
    size: u64,
}

impl Value {
    pub fn from_value(orig: &Value) -> Value {
        Value::new(orig.pos, orig.size)
    }

    pub fn new(pos: u64, size: u64) -> Value {
        let v = Value {
            pos: pos,
            size: size,
        };
        v
    }
}

#[derive(PartialEq, Eq)]
pub struct Values {
    latest: Value,
}

impl Values {
    pub fn new(pos: u64, size: u64) -> Values {
        Values {
            latest: Value::new(pos, size),
        }
    }

    pub fn push(&mut self, value: Value) {
        self.latest = Value::from_value(&self.latest)
    }

    pub fn get(&self, pos: u64, size: u64) -> &Value {
        return &self.latest;
    }
}
