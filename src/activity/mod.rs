//{"completed":0,"correlation_id":222,"cuda_device_id":0,"dur":140277346507968,"hprof_kind":"cupti_activity","kind":"cupti_kernel3","name":"_Z13matrixMulCUDAILi32EEvPfS0_S0_ii","start":0,"stream_id":7}

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct Kernel3S {
    stream_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct MemcpyS {
    src_kind: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Record {
    #[serde(rename = "cupti_kernel3")] Kernel3(Kernel3S),
    #[serde(rename = "cupti_memcpy")] Memcpy(MemcpyS),
}

type ActivityResult = Result<Record, serde_json::Error>;

pub fn from_value(v: serde_json::Value) -> ActivityResult {
    let r: Record = match serde_json::from_value(v) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(r)
}

#[test]
fn kernel3_test() {
    let data = r#"{
        "completed":0,
        "correlation_id":211,
        "cuda_device_id":0,
        "duration":164290,
        "hprof_kind":"cupti_activity",
        "kind":"cupti_kernel3",
        "name":"_Z13matrixMulCUDAILi32EEvPfS0_S0_ii",
        "start":1522123362755573816,
        "stream_id":7}"#;
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let r: Record = from_value(v).unwrap();
    match r {
        Record::Kernel3(s) => assert_eq!(s.stream_id, 7 as u64),
        _ => panic!("Expected a Memcpy!"),
    }
}

#[test]
fn memcpy_test() {
    let data = r#"{
        "correlation_id":203,
        "cuda_device_id":0,
        "cuda_memcpy_kind":"htod",
        "dst_kind":"device",
        "duration":35040,
        "hprof_kind":"cupti_activity",
        "kind":"cupti_memcpy",
        "runtime_correlation_id":0,
        "src_kind":"pageable",
        "start":1522123362755091634,
        "stream_id":7}"#;
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let r: Record = from_value(v).unwrap();
    match r {
        Record::Memcpy(s) => assert_eq!(s.src_kind, "pageable"),
        _ => panic!("Expected a Kernel3!"),
    }
}
