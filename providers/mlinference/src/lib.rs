#![allow(dead_code)]
use thiserror::Error as ThisError;

use std::{
    collections::{HashMap},
};
use serde::{Deserialize};
use wasmcloud_interface_mlinference::{ Tensor, TensorType, ResultStatus, MlError, InferenceOutput };

mod bindle_loader;
pub use bindle_loader::{BindleLoader, ModelMetadata};

mod inference;
pub use inference::{
    ExecutionTarget,
    Graph, GraphEncoding, GraphExecutionContext, 
    TractEngine, ModelState, TType, InferenceEngine};

mod settings;
pub use settings::{load_settings, ModelSettings};

mod hashmap_ci;
pub (crate) use hashmap_ci::make_case_insensitive;

pub type BindlePath = String;
pub type ModelName = String;
pub type ModelZoo = HashMap<ModelName, ModelContext>;

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct ModelContext {
    pub bindle_url: BindlePath,
    pub graph_encoding: GraphEncoding,
    pub execution_target: ExecutionTarget,
    pub tensor_type: TType,
    pub graph_execution_context: GraphExecutionContext,
    pub graph: Graph
}

impl ModelContext {

    /// load metadata
    pub fn load_metadata(&mut self, metadata: ModelMetadata) -> Result<&ModelContext, Error> 
    {
        self.graph_encoding = match metadata.graph_encoding.as_str() {
            "ONNX" => Ok(GraphEncoding(GraphEncoding::GRAPH_ENCODING_ONNX)),
            _      => Err(())
        }.map_err(|_| Error::InvalidParameter("invalid 'graph_encoding'".to_string()))?;

        self.tensor_type = match metadata.tensor_type.as_str() {
            "F16" => Ok(TType(TType::F16)),
            "F32" => Ok(TType(TType::F32)),
             "U8" => Ok(TType(TType::U8)),
            "I32" => Ok(TType(TType::I32)),
               _  => Err(()),
        }.map_err(|_| Error::InvalidParameter("invalid 'tensor_type'".to_string()))?;
 
        self.execution_target = match metadata.execution_target.as_str() {
            "CPU" => Ok(ExecutionTarget(ExecutionTarget::EXECUTION_TARGET_CPU)),
            "GPU" => Ok(ExecutionTarget(ExecutionTarget::EXECUTION_TARGET_GPU)),
            "TPU" => Ok(ExecutionTarget(ExecutionTarget::EXECUTION_TARGET_TPU)),
               _  => Err(()),
        }.map_err(|_| Error::Settings("invalid 'execution_target'".to_string()))?;

        Ok(self)
    }
}

/// generates an error default ResultStatus
pub fn get_result_status(ml_error_option: Option<MlError>) -> ResultStatus {
    let with_error = ml_error_option.is_some();
    
    ResultStatus {
        has_error: with_error,
        error: ml_error_option
    }
}

/// generates an error default ResultStatus
pub fn get_default_inference_result(ml_error: Option<MlError>) -> InferenceOutput {
    InferenceOutput {
        result: get_result_status(ml_error),
        tensor: Tensor {
            ttype: TensorType{ ttype: 0},
            dimensions: vec![],
            data: vec![]
        }
    }
}


/// errors generated by this crate
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("problem reading settings: {0}")]
    Settings(String),

    #[error("provider startup: {0}")]
    Init(String)
}