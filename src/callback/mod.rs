extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct CudaMallocS {
    calling_tid: u64,
    context_uid: u64,
    id: u64,
    ptr: u64,
    size: u64,
    wall_end: u64,
    wall_start: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum Record {
    #[serde(rename = "cudaMalloc")] CudaMalloc(CudaMallocS),
}

type DriverApiResult = Result<Record, serde_json::Error>;

pub fn from_value(v: serde_json::Value) -> DriverApiResult {
    let r: Record = match serde_json::from_value(v) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(r)
}

#[test]
fn cuda_malloc_test() {
    use std::io::BufReader;
    use callback::*;
    let data = r#"{"calling_tid":1390,
                    "context_uid":0,
                    "correlation_id":200,
                    "hprof_kind":"cupti_callback",
                    "id":2,
                    "name":"cudaMalloc",
                    "ptr":140277085896704,
                    "size":409600,
                    "symbol_name":"",
                    "wall_end":1522106423006283946,
                    "wall_start":1522106422797168222}"#;
    let mut reader = BufReader::new(data.as_bytes());
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let r: Record = from_value(v).unwrap();
    match r {
        Record::CudaMalloc(s) => assert_eq!(s.id, 2 as u64),
        _ => panic!("Expected a CudaMalloc!"),
    }
}
