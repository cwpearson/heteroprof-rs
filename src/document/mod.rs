use pdg;
use activity;
use callback;
use cuda;
use cublas;
use cudnn;
use nccl;

pub struct Document {
    pub activities: Vec<activity::Record>,
    pub apis: Vec<callback::Record>,
    pub cudnn_calls: Vec<cudnn::Record>,
    pub nccl_calls: Vec<nccl::Record>,
    pub cublas_calls: Vec<cublas::Record>,
}
