extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
struct AddressSpaceRaw {
    ty: String,
    initialized: String,
}

enum Type {
    UVA,
}

struct AddressSpace {
    ty: Type,
    initialized: String,
}

#[derive(Serialize, Deserialize)]
struct AllocationRaw {
    id: String,
    pos: String,
    size: String,
    addrsp: AddressSpaceRaw,
}

#[derive(Serialize, Deserialize)]
struct AllocationWrapperRaw {
    allocation: AllocationRaw,
}

pub struct Allocation {
    id: u64,
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

    let a = Allocation {
        id: awr.allocation.id.parse::<u64>().unwrap(),
    };

    Ok(a)
}

#[test]
fn allocation_test() {
    use std::io::BufReader;
    let data = r#"{"allocation":
                    {"id":"69268522399040",
                    "pos":"1099889124352",
                    "size":"112",
                    "addrsp":"{\"type\":\"uva\"}\n",
                    "mem":"pageable",
                    "loc":"{\"type\":\"cuda\",\"id\":\"0\"}\n"}
                }"#;
    let mut reader = BufReader::new(data.as_bytes());
    let a: AllocationWrapperRaw = serde_json::from_str(&data).unwrap();
}
