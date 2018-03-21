extern crate serde;
extern crate serde_json;

use serde_json::{Error, Value};

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct Compute {
    kind: String,
    cuda_device_id: String,
}

#[derive(Serialize, Deserialize)]
struct Transfer {
    kind: String,
    cuda_device_id: String,
}


#[test]
fn compute_test() {
    let data = r#"{"cuda_device_id":"0",
                   "kind":"cupti_kernel3",
                   "name":"_ZN7mshadow4cuda13MapPlanKernelINS_2sv6savetoELi8ENS_4expr4PlanINS_6TensorINS_3gpuELi2EfEEfEENS5_INS4_9ScalarExpIfEEfEEEEvT1_jNS_5ShapeILi2EEET2_",
                   "start":"1.5215767292957988e+18",
                   "dur":"3968",
                   "completed":"0",
                   "stream_id":"15",
                   "correlation_id":"1825"}"#;
    let c: Compute = serde_json::from_str(data).unwrap();
}

struct Document {}
