extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
struct ApiRaw {
    api: Api,
}

#[derive(Serialize, Deserialize)]
pub struct Api {
    correlation_id: u64,
    device: u64,
    id: u64,
    inputs: Vec<u64>,
    outputs: Vec<u64>,
    name: String,
    symbolname: String,
    wall_end: u64,
    wall_start: u64,
}

type ApiResult = Result<Api, serde_json::Error>;

pub fn from_value(v: serde_json::Value) -> ApiResult {
    let ar: ApiRaw = match serde_json::from_value(v) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };
    Ok(ar.api)
}

#[test]
fn api_test() {
    use std::io::BufReader;
    let data = r#"{"api":
                    {"correlation_id":1292,
                    "device":0,
                    "id":26305,
                    "inputs":[10],
                    "kv":{"dstCount":"112","kind":"cudaMemcpyHostToDevice","srcCount":"112"},
                    "name":"cudaMemcpy",
                    "outputs":[11],
                    "symbolname":"",
                    "wall_end":1521742255038253985,
                    "wall_start":1521742255037977930}
                }"#;
    let mut reader = BufReader::new(data.as_bytes());
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let a: Api = from_value(v).unwrap();
    assert_eq!(a.id, 26305 as u64);
}
