extern crate serde;
extern crate serde_json;

use serde_json::{Error, Value};

#[macro_use]
extern crate serde_derive;

use std::io::BufRead;

#[derive(Serialize, Deserialize)]
pub struct Compute {
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

#[derive(Serialize, Deserialize)]
pub struct Transfer {
    kind: String,
    cuda_device_id: String,
}

#[test]
fn transfer_test() {
    let data = r#"{"cuda_device_id":"1",
                   "kind":"cupti_memcpy",
                   "cuda_memcpy_kind":"htod",
                   "src_kind":"pageable",
                   "dst_kind":"device",
                   "start":"1.5215767305709568e+18",
                   "dur":"1056",
                   "stream_id":"35",
                   "correlation_id":"4031",
                   "runtime_correlation_id":"0"}"#;
    let t: Transfer = serde_json::from_str(data).unwrap();
}

pub enum DecoderError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
}

pub struct Document {
    transfers: Vec<Transfer>,
    computes: Vec<Compute>,
}

impl Document {
    fn new() -> Document {
        return Document {
            computes: vec![],
            transfers: vec![],
        };
    }
}

type DecoderResult<T> = Result<T, DecoderError>;

pub fn decode_document<BR: BufRead + ?Sized>(br: &mut BR) -> DecoderResult<Document> {
    let mut doc = Document::new();
    let mut line = String::new();
    loop {
        let bytes = match br.read_line(&mut line) {
            Ok(bytes) => bytes,
            Err(e) => return Err(DecoderError::IoError(e)),
        };
        let mut v: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => return Err(DecoderError::JsonError(e)),
        };

        match v.get_mut("compute") {
            None => (),
            Some(v) => {
                let c: Compute = serde_json::from_value(v.take()).unwrap();
                doc.computes.push(c);
            }
        }
        println!("{}", bytes);
    }
}
