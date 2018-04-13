pub mod pdg;
mod activity;
mod callback;
mod cuda;
mod cublas;
mod cudnn;
mod nccl;

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

    pub fn add_activity(&mut self, r: activity::Record) {
        self.activities.push(r);
    }
    pub fn add_api(&mut self, r: callback::Record) {
        self.apis.push(r);
    }
}

type DecoderResult<T> = Result<T, DecoderError>;

pub fn decode_document<BR: BufRead + ?Sized>(br: &mut BR) -> DecoderResult<Document> {
    let mut doc = Document::new();

    let stream = serde_json::Deserializer::from_reader(br).into_iter::<serde_json::Value>();

    for val in stream {
        let mut val = match val {
            Err(e) => return Err(DecoderError::JsonError(e)),
            Ok(val) => val,
        };

        let kind_check_val = val.clone();

        if let serde_json::Value::Object(obj) = kind_check_val {
            if let Some(kind) = obj.get("hprof_kind") {
                if kind.as_str().unwrap() == "cupti_callback" {
                    if let Ok(a) = callback::from_value(val.take()) {
                        doc.add_api(a);
                        continue;
                    }
                } else if kind == "cupti_activity" {
                    if let Ok(a) = activity::from_value(val.take()) {
                        doc.add_activity(a);
                        continue;
                    }
                }
            }
        }
    }

    Ok(doc)
}

#[test]
fn document_test() {
    use std::io::BufReader;
    let data = r#"{"calling_tid":4063,"context_uid":1,"correlation_id":2308,"func":4216851,"hprof_kind":"cupti_callback","id":2106,"name":"cudaLaunch","params":[],"symbol_name":"","wall_end":1522123362821404237,"wall_start":1522123362821383713}
{"block_dim":{"x":32,"y":32,"z":1},"calling_tid":4063,"context_uid":1,"correlation_id":2309,"grid_dim":{"x":20,"y":10,"z":1},"hprof_kind":"cupti_callback","id":2107,"name":"cudaConfigureCall","shared_mem":0,"stream":0,"symbol_name":"","wall_end":1522123362821434714,"wall_start":1522123362821431121}
{"arg":140732184584328,"calling_tid":4063,"context_uid":1,"correlation_id":2310,"hprof_kind":"cupti_callback","id":2108,"name":"cudaSetupArgument","offset":0,"size":8,"symbol_name":"","wall_end":1522123362821468267,"wall_start":1522123362821464959}
{"arg":140732184584320,"calling_tid":4063,"context_uid":1,"correlation_id":2311,"hprof_kind":"cupti_callback","id":2109,"name":"cudaSetupArgument","offset":8,"size":8,"symbol_name":"","wall_end":1522123362821488236,"wall_start":1522123362821485927}
{"arg":140732184584312,"calling_tid":4063,"context_uid":1,"correlation_id":2312,"hprof_kind":"cupti_callback","id":2110,"name":"cudaSetupArgument","offset":16,"size":8,"symbol_name":"","wall_end":1522123362821507178,"wall_start":1522123362821504954}
{"arg":140732184584308,"calling_tid":4063,"context_uid":1,"correlation_id":2313,"hprof_kind":"cupti_callback","id":2111,"name":"cudaSetupArgument","offset":24,"size":4,"symbol_name":"","wall_end":1522123362821525725,"wall_start":1522123362821523513}
{"arg":140732184584304,"calling_tid":4063,"context_uid":1,"correlation_id":2314,"hprof_kind":"cupti_callback","id":2112,"name":"cudaSetupArgument","offset":28,"size":4,"symbol_name":"","wall_end":1522123362821543999,"wall_start":1522123362821541859}
{"calling_tid":4063,"context_uid":1,"correlation_id":2315,"func":4216851,"hprof_kind":"cupti_callback","id":2113,"name":"cudaLaunch","params":[],"symbol_name":"","wall_end":1522123362821600310,"wall_start":1522123362821578241}
{"calling_tid":4063,"context_uid":1,"correlation_id":2319,"count":819200,"cuda_memcpy_kind":2,"dst":140481688645648,"hprof_kind":"cupti_callback","id":2114,"name":"cudaMemcpy","src":140481569931264,"symbol_name":"","wall_end":1522123362822187482,"wall_start":1522123362821800712}
{"correlation_id":203,"cuda_device_id":0,"cuda_memcpy_kind":"htod","dst_kind":"device","duration":35040,"hprof_kind":"cupti_activity","kind":"cupti_memcpy","runtime_correlation_id":0,"src_kind":"pageable","start":1522123362755091634,"stream_id":7}
{"correlation_id":204,"cuda_device_id":0,"cuda_memcpy_kind":"htod","dst_kind":"device","duration":67200,"hprof_kind":"cupti_activity","kind":"cupti_memcpy","runtime_correlation_id":0,"src_kind":"pageable","start":1522123362755231796,"stream_id":7}
{"completed":0,"correlation_id":211,"cuda_device_id":0,"duration":164290,"hprof_kind":"cupti_activity","kind":"cupti_kernel3","name":"_Z13matrixMulCUDAILi32EEvPfS0_S0_ii","start":1522123362755573816,"stream_id":7}
{"completed":0,"correlation_id":222,"cuda_device_id":0,"duration":162818,"hprof_kind":"cupti_activity","kind":"cupti_kernel3","name":"_Z13matrixMulCUDAILi32EEvPfS0_S0_ii","start":1522123362755974461,"stream_id":7}
{"completed":0,"correlation_id":229,"cuda_device_id":0,"duration":161378,"hprof_kind":"cupti_activity","kind":"cupti_kernel3","name":"_Z13matrixMulCUDAILi32EEvPfS0_S0_ii","start":1522123362756158143,"stream_id":7}
{"completed":0,"correlation_id":236,"cuda_device_id":0,"duration":162658,"hprof_kind":"cupti_activity","kind":"cupti_kernel3","name":"_Z13matrixMulCUDAILi32EEvPfS0_S0_ii","start":1522123362756379265,"stream_id":7}
"#;
    let mut reader = BufReader::new(data.as_bytes());
    let doc: Document = decode_document(&mut reader).unwrap();
    assert_eq!(doc.apis.len(), 7);
}
