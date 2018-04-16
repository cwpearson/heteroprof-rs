extern crate serde;
extern crate serde_json;

use std::cmp::Ordering;
use callback;
use activity;

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
    pub duration: f64,
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

// pub fn from_callback(v: &callback::Record) {
//     match v {
//         &callback::Record::CudaMalloc(ref s) => {
//             //Do logic for a new allocation
//             let mut r = Compute {
//                 correlation_id: s.correlation_id,
//                 cuda_device_id: s.cuda_device_id,
//                 kind: s.kind,
//                 name: s.name,
//                 start: s.start,
//                 duration: s.end,
//                 completed: 1,
//                 stream_id: s.stream_id,
//             };
//         }
//         _ => {
//             //Don't need to do anything, as we are only interested in memory transfers
//         }
//     }
// }

// pub fn from_activity(v: &activity::Record) {
//     match v {
//         &activity::Record::Kernel3(ref s) => {
//             //Do something for a kernel launch
//         }
//         _ => {
//             //Uhhhhh
//         }
//     }
// }

#[test]
fn api_test() {
    use std::io::BufReader;
    let data = r#"{"compute":
                {"completed":0.0,
                "correlation_id":1858,
                "cuda_device_id":0,
                "duration":2112.0,
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
