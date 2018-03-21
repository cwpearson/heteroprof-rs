extern crate serde;
extern crate serde_json;

use serde_json::Value;

#[macro_use]
extern crate serde_derive;

use std::io::BufRead;
// use std::fmt::Debug;
use std::cmp::Ordering;

#[derive(Serialize, Deserialize)]
struct serdes_compute {
    kind: String,
    cuda_device_id: String,
    name: String,
    start: String,
    dur: String,
    completed: String,
    stream_id: String,
    correlation_id: String,
}

pub struct Compute {
    pub kind: String,
    pub cuda_device_id: u64,
    pub name: String,
    pub start: f64,
    pub dur: f64,
    pub completed: f64,
    pub stream_id: u64,
    pub correlation_id: u64,
}

impl Compute {
    fn from_serdes_compute(sc: &serdes_compute) -> Compute {
        let kind = sc.kind.clone();
        let cuda_device_id = sc.cuda_device_id.parse::<u64>().unwrap();
        let name = sc.name.clone();
        let start = sc.start.parse::<f64>().unwrap();
        let dur = sc.dur.parse::<f64>().unwrap();
        let completed = sc.completed.parse::<f64>().unwrap();
        let stream_id = sc.stream_id.parse::<u64>().unwrap();
        let correlation_id = sc.correlation_id.parse::<u64>().unwrap();
        return Compute {
            kind: kind,
            cuda_device_id: cuda_device_id,
            name: name,
            start: start,
            dur: dur,
            completed: completed,
            stream_id: stream_id,
            correlation_id: correlation_id,
        };
    }

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
    let sc: serdes_compute = serde_json::from_str(data).unwrap();
    let c = Compute::from_serdes_compute(&sc);
    assert_eq!(c.kind, "cupti_kernel3");
}

#[derive(Serialize, Deserialize)]
struct serdes_transfer {
    kind: String,
    cuda_device_id: String,
    src_kind: String,
    dst_kind: String,
    start: String,
    dur: String,
    stream_id: String,
    correlation_id: String,
    runtime_correlation_id: String,
}

pub struct Transfer {
    pub kind: String,
    pub cuda_device_id: u64,
    pub src_kind: String,
    pub dst_kind: String,
    pub start: f64,
    pub dur: f64,
    pub stream_id: u64,
    pub correlation_id: u64,
    pub runtime_correlation_id: u64,
}

impl Transfer {
    fn from_serdes_transfer(st: &serdes_transfer) -> Transfer {
        let kind = st.kind.clone();
        let src_kind = st.src_kind.clone();
        let dst_kind = st.dst_kind.clone();
        let cuda_device_id = st.cuda_device_id.parse::<u64>().unwrap();
        let start = st.start.parse::<f64>().unwrap();
        let dur = st.dur.parse::<f64>().unwrap();
        let stream_id = st.stream_id.parse::<u64>().unwrap();
        let correlation_id = st.correlation_id.parse::<u64>().unwrap();
        let runtime_correlation_id = st.runtime_correlation_id.parse::<u64>().unwrap();
        return Transfer {
            kind: kind,
            cuda_device_id,
            src_kind,
            dst_kind,
            start,
            dur,
            stream_id,
            correlation_id,
            runtime_correlation_id,
        };
    }

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
    let st: serdes_transfer = serde_json::from_str(data).unwrap();
    let t = Transfer::from_serdes_transfer(&st);
}

#[derive(Debug)]
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

    pub fn computes(&self) -> &Vec<Compute> {
        return &self.computes;
    }
    pub fn computes_mut(&mut self) -> &mut Vec<Compute> {
        return &mut self.computes;
    }

    pub fn transfers(&self) -> &Vec<Transfer> {
        return &self.transfers;
    }

    pub fn transfers_mut(&mut self) -> &mut Vec<Transfer> {
        return &mut self.transfers;
    }
}

type DecoderResult<T> = Result<T, DecoderError>;

pub fn decode_document<BR: BufRead + ?Sized>(br: &mut BR) -> DecoderResult<Document> {
    let mut doc = Document::new();

    let stream = serde_json::Deserializer::from_reader(br).into_iter::<Value>();

    for v in stream {
        let mut v = match v {
            Ok(v) => v,
            Err(e) => return Err(DecoderError::JsonError(e)),
        };
        match v.get_mut("compute") {
            None => (),
            Some(v) => {
                let sc: serdes_compute = serde_json::from_value(v.take()).unwrap();
                let c = Compute::from_serdes_compute(&sc);
                doc.computes.push(c);
                continue;
            }
        }
        match v.get_mut("transfer") {
            None => (),
            Some(v) => {
                let st: serdes_transfer = serde_json::from_value(v.take()).unwrap();
                let t = Transfer::from_serdes_transfer(&st);
                doc.transfers.push(t);
                continue;
            }
        }
    }

    Ok(doc)
}

#[test]
fn document_test() {
    use std::io::BufReader;
    let data = r#"{"compute":
                    {"cuda_device_id":"0",
                    "kind":"cupti_kernel3",
                    "name": "_ZN7mshadow4cuda13MapPlanKernelINS_2sv6savetoELi8ENS_4expr4PlanINS_6TensorINS_3gpuELi2EfEEfEENS5_INS4_9ScalarExpIfEEfEEEEvT1_jNS_5ShapeILi2EEET2_",
                    "start":"1.5215767292957988e+18",
                    "dur":"3968",
                    "completed":"0",
                    "stream_id":"15",
                    "correlation_id":"1825"}
                  }"#;
    let mut reader = BufReader::new(data.as_bytes());
    let doc: Document = decode_document(&mut reader).unwrap();
    assert_eq!(doc.computes[0].cuda_device_id, 0);
}
