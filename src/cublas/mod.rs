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
            pub context_uid: u64,
            pub symbol_name: String,
            pub handle: u64,
            pub input_vector: [u64],
            pub output_vector: [u64],
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

#[derive(Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum Record {
    #[serde(rename = "cublasCreate")] CublasCreate(CublasCreateS),
    #[serde(rename = "cublasDestroy")] CublasDestroy(CublasDestroyS),
    #[serde(rename = "cublasDgemm")] CublasDgemm(CublasDgemmS),
    #[serde(rename = "cublasDgemv")] CublasDgemv(CublasDgemvS),
    #[serde(rename = "cublasSasum")] CublasSasum(CublasSasumS),
    #[serde(rename = "cublasSaxpy")] CublasSaxpy(CublasSaxpyS),
    #[serde(rename = "cublasSdot")] CublasSdot(CublasSdotS),
    #[serde(rename = "cublasSgemm")] CublasSgemm(CublasSgemmS),
    #[serde(rename = "cublasSgemv")] CublasSgemv(CublasSgemvS),
    #[serde(rename = "cublasSscal")] CublasSscal(CublasSscalS),   
}

type RecordResult = Result<Record, serde_json::Error>;

pub fn from_value(v: serde_json::Value) -> RecordResult {
    let r: Record = match serde_json::from_value(v) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(r)
}