extern crate serde;
extern crate serde_json;


macro_rules! add_common_fields {
    (pub struct $name:ident { $( pub $field:ident: $ty:ty ),* $(,)* }) => {
        #[derive(Serialize, Deserialize)]
        pub struct $name {
            pub calling_tid: u64,
            pub wall_start: u64,
            pub wall_end: u64,
            pub id: u64,
            pub handle: u64,
            $( pub $field: $ty ),*
        }
    };
}

//Example on how to add common fields.
//Not directly applicable to this
/*
add_common_fields!(
pub struct CudaConfigureCallS {
    pub grid_dim: Dim3<u64>,
    pub block_dim: Dim3<u64>,
    pub shared_mem: u64,
    pub stream: u64,
}
);*/

add_common_fields!(
pub struct NcclAllReduceS {
    pub input_vector: [u64; 1],
    pub output_vector: [u64; 1],
}
);

add_common_fields!(
pub struct NcclBcastS {
    pub input_vector: [u64; 1],
    pub output_vector: [u64; 1],
}
);

add_common_fields!(
pub struct NcclCommDestroyS {
}
);

add_common_fields!(
pub struct NcclCommInitAllS {
    pub gpu: u64,
    pub nccl_comm: u64,
}
);

add_common_fields!(
pub struct NcclCommInitRankS {
    pub gpu: u64,
    pub nccl_comm :u64
}
);



#[derive(Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum Record {
    #[serde(rename = "ncclAllReduce")] NcclAllReduce(NcclAllReduceS),
    #[serde(rename = "ncclBcast")] NcclBcast(NcclBcastS),
    #[serde(rename = "ncclCommDestroy")] NcclCommDestroy(NcclCommDestroyS),
    #[serde(rename = "ncclCommInitAll")] NcclCommInitAll(NcclCommInitAllS),
    #[serde(rename = "ncclCommInitRank")] NccCommInitRank(NcclCommInitRankS),
}

// #[test]
// fn cublas_cublas_create_test() {
//     let data = r#"{"calling_tid":11358,
//     "cublas_handle":70368511725280,
//     "hprof_kind":"cublas","id":1,
//     "name":"cublasCreate",
//     "wall_end":1522732322551348279,
//     "wall_start":1522732322054381008}"#;
//     let v: serde_json::Value = serde_json::from_str(&data).unwrap();
//     let r: Record = from_value(v).unwrap();
//     match r {
//         Record::CublasCreate(s) => assert_eq!(s.id, 1 as u64),
//         _ => panic!("Expected a CudaSetupArgument!"),
//     }
// }

// #[test]
// fn cublas_cublas_destroy_test() {
//      let data = r#"{"calling_tid":11358,
//      "handle":69269201188720,
//      "hprof_kind":"cublas",
//      "id":8,"input_vector":[],
//      "name":"cublasDestroy",
//      "output_vector":[],
//      "wall_end":1522732322554073510,
//      "wall_start":1522732322551483898}"#;
//     let v: serde_json::Value = serde_json::from_str(&data).unwrap();
//     let r: Record = from_value(v).unwrap();
//     match r {
//         Record::CublasDestroy(s) => assert_eq!(s.handle, 69269201188720 as u64),
//         _ => panic!("Expected a CudaSetupArgument!"),
//     }
// }

type RecordResult = Result<Record, serde_json::Error>;

pub fn from_value(v: serde_json::Value) -> RecordResult {
    let r: Record = match serde_json::from_value(v) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(r)
}