extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
struct ComputeRaw {
    compute: Compute,
}

#[derive(Serialize, Deserialize)]
pub struct Compute {
    correlation_id: u64,
    cuda_device_id: u64,
    kind: String,
    name: String,
    start: f64,
    dur: f64,
    completed: f64,
    stream_id: u64,
}

type ComputeResult = Result<Compute, serde_json::Error>;

pub fn from_value(v: serde_json::Value) -> ComputeResult {
    let r: ComputeRaw = match serde_json::from_value(v) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    Ok(r.compute)
}

#[test]
fn api_test() {
    use std::io::BufReader;
    let data = r#"{"compute":
                    {"cuda_device_id":"0",
                    "kind":"cupti_kernel3",
                    "name":"_ZN7mshadow4cuda13MapPlanKernelINS_2sv6savetoELi8ENS_4expr4PlanINS_6TensorINS_3gpuELi2EfEEfEENS5_INS4_9ScalarExpIfEEfEEEEvT1_jNS_5ShapeILi2EEET2_",
                    "start":"1.5217422554930066e+18",
                    "dur":"3840",
                    "completed":"0",
                    "stream_id":"15",
                    "correlation_id":"1827"}
                }"#;
    let mut reader = BufReader::new(data.as_bytes());
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let c: Compute = from_value(v).unwrap();
    assert_eq!(c.correlation_id, 1827 as u64);
}
