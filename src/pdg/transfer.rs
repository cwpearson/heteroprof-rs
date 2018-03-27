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

type TransferResult = Result<Transfer, serde_json::Error>;

pub fn from_value(v: serde_json::Value) -> TransferResult {
    let r: TransferRaw = match serde_json::from_value(v) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    Ok(r.transfer)
}

#[test]
fn transfer_test() {
    use std::io::BufReader;
    let data = r#"{"transfer":
        {"correlation_id":4031,
        "cuda_device_id":1,
        "cuda_memcpy_kind":"htod",
        "dst_kind":"device",
        "dur":1056.0,
        "kind":"cupti_memcpy",
        "kv":{},"runtime_correlation_id":0,"src_kind":"pageable",
        "start":1.5217442373738993e+18,
        "stream_id":35}
    }"#;
    let mut reader = BufReader::new(data.as_bytes());
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let t: Transfer = from_value(v).unwrap();
    assert_eq!(t.stream_id, 35 as u64);
}
