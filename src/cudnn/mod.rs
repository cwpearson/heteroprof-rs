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
            pub cudnn_handle: u64,
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
pub struct CudnnActivationBackwardS {
    pub input_vector: [u64; 10],
    pub output_vector: [u64; 1],
}
);

add_common_fields!(
pub struct CudnnActivationForwardS {
    pub input_vector: [u64; 6],
    pub output_vector: [u64; 1],
}
);

add_common_fields!(
pub struct CudnnAddTensorS {
    pub input_vector: [u64; 6],
    pub output_vector: [u64; 1],
}
);

add_common_fields!(
pub struct CudnnConvolutionBackwardBiasS {
    pub input_vector: [u64; 5],
    pub output_vector: [u64; 1],
}
);

add_common_fields!(
pub struct CudnnConvolutionBackwardDataS {
    pub input_vector: [u64; 12],
    pub output_vector: [u64; 1],
}
);

add_common_fields!(
pub struct CudnnConvolutionBackwardFilterS {
    pub input_vector: [u64; 12],
    pub output_vector: [u64; 1],
}
);

add_common_fields!(
pub struct CudnnConvolutionForwardS {
    pub input_vector: [u64; 12],
    pub output_vector: [u64; 1],
}
);

add_common_fields!(
pub struct CudnnCreateS {
}
);

add_common_fields!(
pub struct CudnnDestroyS {
}
);

add_common_fields!(
pub struct CudnnPoolingForwardS {
    pub input_vector: [u64; 6],
    pub output_vector: [u64; 1]
}
);

add_common_fields!(
pub struct CudnnSoftmaxForwardS {
    pub input_vector: [u64; 6],
    pub output_vector: [u64; 1]
}
);

#[derive(Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum Record {
    #[serde(rename = "cudnnActivationBackward")] CudnnActivationBackward(CudnnActivationBackwardS),
    #[serde(rename = "cudnnActivationForward")] CudnnActivationForward(CudnnActivationForwardS),
    #[serde(rename = "cudnnAddTensor")] CudnnAddTensor(CudnnAddTensorS),
    #[serde(rename = "cudnnConvolutionBackwardBias")] CudnnConvolutionBackwardBias(CudnnConvolutionBackwardBiasS),
    #[serde(rename = "cudnnConvolutionBackwardData")] CudnnConvolutionBackwardData(CudnnConvolutionBackwardDataS),
    #[serde(rename = "cudnnConvolutionBackwardFilter")] CudnnConvolutionBackwardFilter(CudnnConvolutionBackwardFilterS),
    #[serde(rename = "cudnnConvolutionForward")] CudnnConvolutionForward(CudnnConvolutionForwardS),
    #[serde(rename = "cudnnCreate")] CudnnCreate(CudnnCreateS),
    #[serde(rename = "cudnnDestroy")] CudnnDestroy(CudnnDestroyS),
    #[serde(rename = "cudnnPoolingForward")] CudnnPoolingForward(CudnnPoolingForwardS),
    #[serde(rename = "cudnnSoftmaxForward")] CudnnSoftmaxForward(CudnnSoftmaxForwardS),   
}

#[test]
fn cudnn_add_tensor_test() {
    let data = r#"{"calling_tid":14585,
    "cudnn_handle":69268213715040,
    "hprof_kind":"cudnn",
    "id":568964,"input_vector":[70367866280808,69268213681232,1099882827264,70367866280808,69268213665424,1099891212288],
    "name":"cudnnAddTensor",
    "output_vector":[1099891212288],
    "wall_end":1522691253936525384,"wall_start":1522691253935481877}"#;
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let r: Record = from_value(v).unwrap();
    match r {
        Record::CudnnAddTensor(s) => assert_eq!(s.id, 568964 as u64),
        _ => panic!("Expected a CudaSetupArgument!"),
    }
}

#[test]
fn cudnn_convolution_backward_bias_test() {
    let data = r#"{"calling_tid":14585,
    "cudnn_handle":69269000513424,
    "hprof_kind":"cudnn","id":775,
    "input_vector":[70367866276052,69268418360352,1099906744320,70367866276056,69268422887520],
    "name":"cudnnConvolutionBackwardBias",
    "output_vector":[1099882829312],
    "wall_end":1522691204227418005,
    "wall_start":1522691204226705392}"#;
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let r: Record = from_value(v).unwrap();
    match r {
        Record::CudnnConvolutionBackwardBias(s) => assert_eq!(s.input_vector, [70367866276052,69268418360352,1099906744320,70367866276056,69268422887520]),
        _ => panic!("Expected a CudaSetupArgument!"),
    }
}

#[test]
fn cudnn_convolution_backward_data_test(){
    let data = r#"{"calling_tid":14585,
    "cudnn_handle":69269000513424,
    "hprof_kind":"cudnn",
    "id":800,"input_vector":[70367866276052,69268418359456,1099889776640,69268418360352,1099906744320,69268418359968,1,1099897522176,3464,70367866276056,69268418359776,1099896225792],
    "name":"cudnnConvolutionBackwardData",
    "output_vector":[1099896225792],
    "wall_end":1522691204229896916,"wall_start":1522691204228924178}"#;
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let r: Record = from_value(v).unwrap();
    match r {
        Record::CudnnConvolutionBackwardData(s) => assert_eq!(s.cudnn_handle, 69269000513424 as u64),
        _ => panic!("Expected a CudaSetupArgument!"),
    }
}

#[test]
fn cudnn_activation_backward_test() {
     let data = r#"{"calling_tid":14585,
     "cudnn_handle":69269000513424,
     "hprof_kind":"cudnn",
     "id":681,
     "input_vector":[69269001368544,70367866276052,69268418360064,1099889648640,69268418360064,1099890310144,69268418360064,1099889520640,70367866276056,69268418360064],
     "name":"cudnnActivationBackward",
     "output_vector":[1099890182144],
     "wall_end":1522691204220833988,
     "wall_start":1522691204219921261}"#;
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let r: Record = from_value(v).unwrap();
    match r {
        Record::CudnnActivationBackward(s) => assert_eq!(s.output_vector, [1099890182144]),
        _ => panic!("Expected a CudaSetupArgument!"),
    }
}


#[test]
fn cudnn_activation_forward_test() {
     let data = r#"{"calling_tid":14585,
     "cudnn_handle":69269000513424,
     "hprof_kind":"cudnn",
     "id":572,"input_vector":[69269001368544,70367866280808,69268418360064,1099889520640,70367866280812,69268418360064],
     "name":"cudnnActivationForward",
     "output_vector":[1099889648640],"wall_end":1522691204212516355,"wall_start":1522691204211743417}"#;
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let r: Record = from_value(v).unwrap();
    match r {
        Record::CudnnActivationForward(s) => assert_eq!(s.output_vector, [1099889648640]),
        _ => panic!("Expected a CudaSetupArgument!"),
    }
}

#[test]
fn cudnn_convolution_backward_filter_test() {
     let data = r#"{"calling_tid":14585,
     "cudnn_handle":69269000513424,
     "hprof_kind":"cudnn","id":785,
     "input_vector":[70367866276052,69268418359776,1099894161408,69268418360352,1099906744320,69268418359968,3,1099897522176,3464,70367866276056,69268418359456,1099889876992],
     "name":"cudnnConvolutionBackwardFilter",
     "output_vector":[1099889876992],"wall_end":1522691204228857240,"wall_start":1522691204227480845}"#;
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let r: Record = from_value(v).unwrap();
    match r {
        Record::CudnnConvolutionBackwardFilter(s) => assert_eq!(s.id, 785 as u64),
        _ => panic!("s.id test failed!")
    }
}

#[test]
fn cudnn_create_test() {
     let data = r#"{"calling_tid":14585,
     "cudnn_handle":70367866297760,
     "hprof_kind":"cudnn","id":414070,
     "input_vector":[],"name":"cudnnCreate","output_vector":[],
     "wall_end":1522691239747659495,"wall_start":1522691239742154224}"#;
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let r: Record = from_value(v).unwrap();
    match r {
        Record::CudnnCreate(s) => assert_eq!(s.id, 414070 as u64),
        _ => panic!("Expected a CudaSetupArgument!"),
    }
}

#[test]
fn cudnn_destroy_test() {
    let data = r#"{"calling_tid":14585,
    "cudnn_handle":69268213715040,"hprof_kind":"cudnn",
    "id":1654084,"input_vector":[],
    "name":"cudnnDestroy","output_vector":[],
    "wall_end":1522691342707328420,"wall_start":1522691342704115464}"#;
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let r: Record = from_value(v).unwrap();
    match r {
        Record::CudnnDestroy(s) => assert_eq!(s.id, 1654084 as u64),
        _ => panic!("Expected a CudaSetupArgument!"),
    }
}

#[test]
fn cudnn_softmax_forward_test() {
    let data = r#"{"calling_tid":14585,
    "cudnn_handle":69269000513424,
    "hprof_kind":"cudnn","id":82,
    "input_vector":[69268422883968,70367866280808,69268418362016,1099891212288,70367866280812,69268418359776],
    "name":"cudnnSoftmaxForward","output_vector":[1099894161408],"wall_end":1522691204170924921,"wall_start":1522691204169859866}"#;
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let r: Record = from_value(v).unwrap();
    match r {
        Record::CudnnSoftmaxForward(s) => assert_eq!(s.output_vector, [1099894161408]),
        _ => panic!("Expected a CudaSetupArgument!"),
    }
}

type RecordResult = Result<Record, serde_json::Error>;

pub fn from_value(v: serde_json::Value) -> RecordResult {
    let r: Record = match serde_json::from_value(v) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(r)
}