extern crate serde;
extern crate serde_json;

macro_rules! add_common_fields {
    (pub struct $name:ident { $( $field:ident: $ty:ty ),* $(,)* }) => {
        #[derive(Serialize, Deserialize)]
        pub struct $name {
            calling_tid: u64,
            wall_start: u64,
            wall_end: u64,
            id: u64,
            context_uid: u64,
            symbol_name: String,
            $( $field: $ty ),*
        }
    };
}

add_common_fields!(
pub struct CudaMallocS {
    ptr: u64,
    size: u64,
}
);

add_common_fields!(
pub struct CudaMemcpyS {
    src: u64,
    count: u64,
    dst: u64,
}
);

add_common_fields!(
pub struct CudaSetupArgumentS {
    offset: u64,
    size: u64,
    arg: u64,
}
);

#[derive(Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum Record {
    #[serde(rename = "cudaMalloc")] CudaMalloc(CudaMallocS),
    #[serde(rename = "cudaMemcpy")] CudaMemcpy(CudaMemcpyS),
    #[serde(rename = "cudaSetupArgument")] CudaSetupArgument(CudaSetupArgumentS),
}

type RecordResult = Result<Record, serde_json::Error>;

pub fn from_value(v: serde_json::Value) -> RecordResult {
    let r: Record = match serde_json::from_value(v) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(r)
}

#[test]
fn cuda_malloc_test() {
    let data = r#"{"calling_tid":1390,
                    "context_uid":0,
                    "correlation_id":200,
                    "hprof_kind":"cupti_callback",
                    "id":2,
                    "name":"cudaMalloc",
                    "ptr":140277085896704,
                    "size":409600,
                    "symbol_name":"",
                    "wall_end":1522106423006283946,
                    "wall_start":1522106422797168222}"#;
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let r: Record = from_value(v).unwrap();
    match r {
        Record::CudaMalloc(s) => assert_eq!(s.id, 2 as u64),
        _ => panic!("Expected a CudaMalloc!"),
    }
}

#[test]
fn cuda_memcpy_test() {
    let data = r#"{"calling_tid":1390,
    "context_uid":1,"correlation_id":204,
    "count":819200,"cuda_memcpy_kind":1,
    "dst":140277086306304,
    "hprof_kind":"cupti_callback",
    "id":6,"name":"cudaMemcpy",
    "src":140277207515152,
    "symbol_name":"",
    "wall_end":1522106423006701111,
    "wall_start":1522106423006592781}"#;
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let r: Record = from_value(v).unwrap();
    match r {
        Record::CudaMemcpy(s) => assert_eq!(s.id, 6 as u64),
        _ => panic!("Expected a CudaMemcpy!"),
    }
}

#[test]
fn cuda_setup_argument_test() {
    let data = r#"{"arg":140723405802000,
    "calling_tid":1390,
    "context_uid":1,
    "correlation_id":210,
    "hprof_kind":"cupti_callback",
    "id":12,
    "name":"cudaSetupArgument",
    "offset":28,
    "size":4,
    "symbol_name":"",
    "wall_end":1522106423007017845,
    "wall_start":1522106423007014860}"#;
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let r: Record = from_value(v).unwrap();
    match r {
        Record::CudaSetupArgument(s) => assert_eq!(s.id, 12 as u64),
        _ => panic!("Expected a CudaSetupArgument!"),
    }
}
