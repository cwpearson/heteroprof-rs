//{"completed":0,"correlation_id":222,"cuda_device_id":0,"dur":140277346507968,"hprof_kind":"cupti_activity","kind":"cupti_kernel3","name":"_Z13matrixMulCUDAILi32EEvPfS0_S0_ii","start":0,"stream_id":7}

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct Kernel3S {}

#[derive(Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum Record {
    #[serde(rename = "cudaMalloc")] Kernel3(Kernel3S),
}

type ActivityResult = Result<Record, serde_json::Error>;

pub fn from_value(v: serde_json::Value) -> ActivityResult {
    let r: Record = match serde_json::from_value(v) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(r)
}
