mod pdg;
mod model;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io::BufRead;

use model::allocation::Allocation;
use model::compute::Compute;
use model::transfer::Transfer;
use model::value;
use model::driver_api::DriverApi;

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

    pub fn add_allocation(&mut self, a: &Allocation) {}
    pub fn add_value(&mut self, v: &value::Value) {}
}

type DecoderResult<T> = Result<T, DecoderError>;

pub fn decode_document<BR: BufRead + ?Sized>(br: &mut BR) -> DecoderResult<Document> {
    let mut doc = Document::new();

    let stream = serde_json::Deserializer::from_reader(br).into_iter::<serde_json::Value>();

    for v in stream {
        let v = match v {
            Ok(v) => v,
            Err(e) => return Err(DecoderError::JsonError(e)),
        };

        let helper = v.clone();
        let o = match helper.as_object() {
            Some(o) => o,
            None => {
                println!("No object: {}", v);
                continue;
            }
        };

        if o.contains_key("allocation") {
            match model::allocation::from_value(v) {
                Err(e) => (),
                Ok(a) => {
                    doc.add_allocation(&a);
                    continue;
                }
            }
        } else if o.contains_key("val") {
            match model::value::from_value(v) {
                Err(e) => (),
                Ok(a) => {
                    doc.add_value(&a);
                    continue;
                }
            }
        } else if o.contains_key("compute") {

        } else if o.contains_key("transfer") {

        }

        // try decoding as allocation
        // match model::allocation::from_value(v) {
        //     Err(e) => (),
        //     Ok(a) => {
        //         doc.add_allocation(&a);
        //         println!("alloc!");
        //         continue;
        //     }
        // };

        // match v.get_mut("compute") {
        //     None => (),
        //     Some(v) => {
        //         let sc: serdes_compute = serde_json::from_value(v.take()).unwrap();
        //         let c = Compute::from_serdes_compute(&sc);
        //         doc.computes.push(c);
        //         continue;
        //     }
        // }
        // match v.get_mut("transfer") {
        //     None => (),
        //     Some(v) => {
        //         let st: serdes_transfer = serde_json::from_value(v.take()).unwrap();
        //         let t = Transfer::from_serdes_transfer(&st);
        //         doc.transfers.push(t);
        //         continue;
        //     }
        // }
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
    assert_eq!(doc.computes.len(), 1);
    assert_eq!(doc.computes[0].cuda_device_id, 0);
}
