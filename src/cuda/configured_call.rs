use std::vec::Vec;

pub struct ConfiguredCall {
    pub args: Vec<u64>,
    pub valid: bool,
}

impl ConfiguredCall {
    pub fn new() -> ConfiguredCall {
        ConfiguredCall {
            args: vec![],
            valid: false,
        }
    }

    pub fn add_arg(&mut self, arg: u64) {
        assert_eq!(self.valid, true);
        self.args.push(arg);
    }

    pub fn finish(&mut self) {
        assert_eq!(self.valid, true);
        self.args.clear();
        self.valid = false;
    }

    pub fn start(&mut self) {
        assert_eq!(self.valid, false);
        self.valid = true;
    }
}
