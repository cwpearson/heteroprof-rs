pub mod pdg;
mod activity;
mod callback;
mod cuda;
mod cublas;
mod cudnn;
mod nccl;
mod statistics;
mod document;

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

impl document::Document {
    fn new() -> document::Document {
        return document::Document {
            activities: vec![],
            apis: vec![],
            cudnn_calls: vec![],
            nccl_calls: vec![],
            cublas_calls: vec![],
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
    pub fn add_cudnn(&mut self, r: cudnn::Record) {
        self.cudnn_calls.push(r);
    }
    pub fn add_cublas(&mut self, r: cublas::Record) {
        self.cublas_calls.push(r);
    }
    pub fn add_nccl(&mut self, r: nccl::Record) {
        self.nccl_calls.push(r);
    }

    pub fn cudnn_calls(&self) -> &Vec<cudnn::Record> {
        return &self.cudnn_calls;
    }

    // pub fn generate_graph(&mut self) {
    //     for entry in &self.apis {
    //         pdg::compute::from_callback(entry);
    //     }
    // }
}

type DecoderResult<T> = Result<T, DecoderError>;

pub fn decode_document<BR: BufRead + ?Sized>(br: &mut BR) -> DecoderResult<document::Document> {
    let mut doc = document::Document::new();

    let stream = serde_json::Deserializer::from_reader(br).into_iter::<serde_json::Value>();

    for val in stream {
        let mut val = match val {
            Err(e) => return Err(DecoderError::JsonError(e)),
            Ok(val) => val,
        };

        let kind_check_val = val.clone();

        if let serde_json::Value::Object(obj) = kind_check_val {
            if let Some(kind) = obj.get("hprof_kind") {
                match kind.as_str().unwrap() {
                    "cupti_callback" => {
                        if let Ok(a) = callback::from_value(val.take()) {
                            doc.add_api(a);
                            continue;
                        }
                    }
                    "cupti_activity" => {
                        if let Ok(a) = activity::from_value(val.take()) {
                            doc.add_activity(a);
                            continue;
                        }
                    }
                    "cudnn" => {
                        if let Ok(a) = cudnn::from_value(val.take()) {
                            doc.add_cudnn(a);
                            continue;
                        }
                    }
                    "cublas" => {
                        if let Ok(a) = cublas::from_value(val.take()) {
                            doc.add_cublas(a);
                            continue;
                        }
                    }
                    "nccl" => {
                        if let Ok(a) = nccl::from_value(val.take()) {
                            doc.add_nccl(a);
                            continue;
                        }
                    }
                    _ => {}
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
    let doc: document::Document = decode_document(&mut reader).unwrap();
    assert_eq!(doc.apis.len(), 9);
}

#[test]
fn document_statistics_test() {
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
    let doc: document::Document = decode_document(&mut reader).unwrap();
    let document_stats = statistics::DocumentStatistics::new(doc);
    document_stats.memory_transfer_statistics();
}

#[test]
fn pdg_graph_generation_test() {
    use std::io::BufReader;
    let data = r#"{"build":"20180402-174617+0000","git":"dirty","version":"0.1.0"}
{"calling_tid":11358,"context_uid":0,"correlation_id":730,"ctx":1100225206112,"hprof_kind":"cupti_callback","id":2,"name":"cuCtxSetCurrent","symbol_name":"","wall_end":1522732322072166606,"wall_start":1522732322072131422}
{"calling_tid":11358,"context_uid":1,"correlation_id":730,"hprof_kind":"cupti_callback","id":3,"name":"cudaFree","ptr":0,"symbol_name":"","wall_end":1522732322546624521,"wall_start":1522732322546409344}
{"calling_tid":11358,"context_uid":1,"correlation_id":743,"hprof_kind":"cupti_callback","id":4,"name":"cudaMalloc","ptr":1099882823680,"size":112,"symbol_name":"","wall_end":1522732322548815154,"wall_start":1522732322547936932}
{"calling_tid":11358,"context_uid":1,"correlation_id":744,"count":112,"cuda_memcpy_kind":1,"dst":1099882823680,"hprof_kind":"cupti_callback","id":5,"name":"cudaMemcpy","src":70368511724768,"symbol_name":"","wall_end":1522732322549018855,"wall_start":1522732322548899498}
{"calling_tid":11358,"context_uid":1,"correlation_id":745,"hprof_kind":"cupti_callback","id":6,"name":"cudaMalloc","ptr":1099882824192,"size":1024,"symbol_name":"","wall_end":1522732322549163887,"wall_start":1522732322549117684}
{"calling_tid":11358,"context_uid":1,"correlation_id":754,"hprof_kind":"cupti_callback","id":7,"name":"cudaMalloc","ptr":1099884920832,"size":32768,"symbol_name":"","wall_end":1522732322550518904,"wall_start":1522732322549978404}
{"calling_tid":11358,"cublas_handle":70368511725280,"hprof_kind":"cublas","id":1,"name":"cublasCreate","wall_end":1522732322551348279,"wall_start":1522732322054381008}
{"calling_tid":11358,"context_uid":1,"correlation_id":763,"hprof_kind":"cupti_callback","id":9,"name":"cudaFree","ptr":1099882823680,"symbol_name":"","wall_end":1522732322551624768,"wall_start":1522732322551616463}
{"calling_tid":11358,"context_uid":1,"correlation_id":773,"hprof_kind":"cupti_callback","id":10,"name":"cudaFree","ptr":1099882824192,"symbol_name":"","wall_end":1522732322552851502,"wall_start":1522732322552844387}
{"calling_tid":11358,"context_uid":1,"correlation_id":783,"hprof_kind":"cupti_callback","id":11,"name":"cudaFree","ptr":1099884920832,"symbol_name":"","wall_end":1522732322554015686,"wall_start":1522732322554008459}
{"calling_tid":11358,"handle":69269201188720,"hprof_kind":"cublas","id":8,"input_vector":[],"name":"cublasDestroy","output_vector":[],"wall_end":1522732322554073510,"wall_start":1522732322551483898}
{"correlation_id":744,"cuda_device_id":0,"cuda_memcpy_kind":"htod","dst_kind":"device","dur":0,"hprof_kind":"cupti_activity","kind":"cupti_memcpy","runtime_correlation_id":0,"src_kind":"pageable","start":0,"stream_id":7}
"#;
    let mut reader = BufReader::new(data.as_bytes());
    let doc: document::Document = decode_document(&mut reader).unwrap();
    let graph = pdg::pdg::from_document(&doc);
}

#[test]
fn pdg_graph_transfer_generation_test() {
    use std::io::BufReader;
        let data = r#"{"build":"20180402-174617+0000","git":"dirty","version":"0.1.0"}
{"calling_tid":11358,"context_uid":1,"correlation_id":743,"hprof_kind":"cupti_callback","id":4,"name":"cudaMalloc","ptr":1099882823680,"size":112,"symbol_name":"","wall_end":1522732322548815154,"wall_start":1522732322547936932}
{"calling_tid":11358,"context_uid":1,"correlation_id":745,"hprof_kind":"cupti_callback","id":6,"name":"cudaMalloc","ptr":70368511724768,"size":1024,"symbol_name":"","wall_end":1522732322549163887,"wall_start":1522732322549117684}
{"calling_tid":11358,"context_uid":1,"correlation_id":744,"count":112,"cuda_memcpy_kind":3,"dst":1099882823680,"hprof_kind":"cupti_callback","id":5,"name":"cudaMemcpy","src":70368511724768,"symbol_name":"","wall_end":1522732322549018855,"wall_start":1522732322548899498}
{"correlation_id":744,"cuda_device_id":0,"cuda_memcpy_kind":"htod","dst_kind":"device","dur":0,"hprof_kind":"cupti_activity","kind":"cupti_memcpy","runtime_correlation_id":0,"src_kind":"pageable","start":0,"stream_id":7}
"#;
    let mut reader = BufReader::new(data.as_bytes());
    let doc: document::Document = decode_document(&mut reader).unwrap();
    let graph = pdg::pdg::from_document(&doc);
    println!("The node count in the graph is {}", graph.graph.node_count());
    println!("The edge count in the graph is {}", graph.graph.edge_count());
    
}