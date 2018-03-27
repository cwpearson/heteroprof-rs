extern crate serde;
extern crate serde_json;

use std::cmp::Ordering;

#[derive(Serialize, Deserialize)]
struct ComputeRaw {
    compute: Compute,
}

#[derive(Serialize, Deserialize)]
pub struct Compute {
    pub correlation_id: u64,
    pub cuda_device_id: u64,
    pub kind: String,
    pub name: String,
    pub start: f64,
    pub dur: f64,
    pub completed: f64,
    pub stream_id: u64,
}

impl Compute {
    pub fn cmp_start(&self, other: &Compute) -> Ordering {
        if self.start == other.start {
            return Ordering::Equal;
        } else if self.start < other.start {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }
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
                {"completed":0.0,
                "correlation_id":1858,
                "cuda_device_id":0,
                "dur":2112.0,
                "kind":"cupti_kernel3",
                "kv":{},"name":"_ZN7mshadow4cuda13MapPlanKernelINS_2sv6savetoELi8ENS_4expr4PlanINS_6TensorINS_3gpuELi2EfEEfEENS5_INS4_9ScalarExpIfEEfEEEEvT1_jNS_5ShapeILi2EEET2_",
                "start":1.5217442345660603e+18,
                "stream_id":14}
            }"#;
    let mut reader = BufReader::new(data.as_bytes());
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let c: Compute = from_value(v).unwrap();
    assert_eq!(c.correlation_id, 1858 as u64);
}
