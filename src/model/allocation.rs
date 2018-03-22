extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
struct AddressSpaceRaw {
    #[serde(rename = "type")] ty: String,
    initialized: String,
}

enum Type {
    UVA,
}

#[derive(Serialize, Deserialize)]
struct AddressSpace {
    ty: String,
    initialized: String,
}

#[derive(Serialize, Deserialize)]
struct AllocationWrapperRaw {
    allocation: Allocation,
}

#[derive(Serialize, Deserialize)]
pub struct Allocation {
    id: u64,
    pos: u64,
    size: u64,
}

#[derive(Debug)]
pub enum AllocationError {
    JsonError(serde_json::Error),
}

type AllocationResult = Result<Allocation, serde_json::Error>;

pub fn from_value(v: &mut serde_json::Value) -> AllocationResult {
    let awr: AllocationWrapperRaw = match serde_json::from_value(v.take()) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };

    let a = awr.allocation;

    Ok(a)
}

#[test]
fn allocation_test() {
    use std::io::BufReader;
    let data = r#"{"allocation":
                    {"addrsp":{"type":"uva"},
                    "id":69268689182064,
                    "loc":{"id":0,"type":"cuda"},
                    "mem":{"type":"pageable"},
                    "pos":1099895410688,
                    "size":2032}
                }"#;
    let mut reader = BufReader::new(data.as_bytes());
    let mut v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let a: Allocation = from_value(&mut v).unwrap();
    assert_eq!(a.id, 69268689182064 as u64);
}
