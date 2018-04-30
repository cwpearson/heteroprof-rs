use activity;
use callback;
use cublas;
use cuda;
use cudnn;
use nccl;
use pdg;

pub struct Document {
    pub activities: Vec<activity::Record>,
    pub apis: Vec<callback::Record>,
    pub cudnn_calls: Vec<cudnn::Record>,
    pub nccl_calls: Vec<nccl::Record>,
    pub cublas_calls: Vec<cublas::Record>,
}
