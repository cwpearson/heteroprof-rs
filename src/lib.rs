mod pdg;
mod activity;
mod callback;
mod cuda;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io::BufRead;

#[derive(Debug)]
pub enum DecoderError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
}

pub struct Document {
    activities: Vec<activity::Record>,
    apis: Vec<callback::Record>,
}

impl Document {
    fn new() -> Document {
        return Document {
            activities: vec![],
            apis: vec![],
        };
    }

    pub fn activities(&self) -> &Vec<activity::Record> {
        return &self.activities;
    }
    pub fn activities_mut(&mut self) -> &mut Vec<activity::Record> {
        return &mut self.activities;
    }

    pub fn apis(&self) -> &Vec<callback::Record> {
        return &self.apis;
    }

    pub fn apis_mut(&mut self) -> &mut Vec<callback::Record> {
        return &mut self.apis;
    }

    pub fn add_activity(&mut self, r: &activity::Record) {}
    pub fn add_api(&mut self, r: &callback::Record) {}
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

        // if o.contains_key("allocation") {
        //     match cuda::allocation::from_value(v) {
        //         Err(e) => (),
        //         Ok(a) => {
        //             doc.add_allocation(&a);
        //             continue;
        //         }
        //     }
        // } else if o.contains_key("val") {
        //     match cuda::value::from_value(v) {
        //         Err(e) => (),
        //         Ok(a) => {
        //             doc.add_value(&a);
        //             continue;
        //         }
        //     }
        if o["hprof_kind"] == "cupti_callback" {
            match callback::from_value(v) {
                Err(e) => (),
                Ok(a) => {
                    doc.add_api(&a);
                    continue;
                }
            }
        } else if o["hprof_kind"] == "cupti_activity" {
            match activity::from_value(v) {
                Err(e) => (),
                Ok(a) => {
                    doc.add_activity(&a);
                    continue;
                }
            }
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
    assert_eq!(doc.apis.len(), 1);
    // assert_eq!(doc.apis[0].cuda_device_id, 0);
}
