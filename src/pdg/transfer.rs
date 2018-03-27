extern crate serde;
extern crate serde_json;

use std::cmp::Ordering;

#[derive(Serialize, Deserialize)]
struct TransferRaw {
    transfer: Transfer,
}

#[derive(Serialize, Deserialize)]
pub struct Transfer {
    pub correlation_id: u64,
    pub cuda_device_id: u64,
    pub kind: String,
    pub start: u64,
    pub dur: u64,
    pub stream_id: u64,
}

impl Transfer {
    pub fn cmp_start(&self, other: &Transfer) -> Ordering {
        if self.start == other.start {
            return Ordering::Equal;
        } else if self.start < other.start {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }
}
