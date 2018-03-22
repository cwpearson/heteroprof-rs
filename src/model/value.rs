extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
struct ValueRaw {
    value: Value,
}

#[derive(Serialize, Deserialize)]
pub struct Value {
    id: u64,
    pos: u64,
    size: u64,
}

type ValueResult = Result<Value, serde_json::Error>;

pub fn from_value(v: serde_json::Value) -> ValueResult {
    let awr: ValueRaw = match serde_json::from_value(v) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };

    let a = awr.value;

    Ok(a)
}

// #[test]
// fn value_test() {
//     use std::io::BufReader;
//     let data = r#"{"allocation":
//                     {"addrsp":{"type":"uva"},
//                     "id":69268689182064,
//                     "loc":{"id":0,"type":"cuda"},
//                     "mem":{"type":"pageable"},
//                     "pos":1099895410688,
//                     "size":2032}
//                 }"#;
//     let mut reader = BufReader::new(data.as_bytes());
//     let v: serde_json::Value = serde_json::from_str(&data).unwrap();
//     let a: Allocation = from_value(v).unwrap();
//     assert_eq!(a.id, 69268689182064 as u64);
// }
