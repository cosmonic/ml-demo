use crate::{ExecutionTarget, GraphEncoding};
use anyhow::{anyhow, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelMetadata {
    /// Model name (optional)
    #[serde(default)]
    pub model_name: Option<String>,

    /// graph encoding
    #[serde(default)]
    pub graph_encoding: GraphEncoding,

    /// execution target
    #[serde(default)]
    pub execution_target: ExecutionTarget,

    /// tensor type
    #[serde(default)]
    pub tensor_type: String,

    /// tensor dimensions in (optional)
    #[serde(default)]
    pub tensor_dimensions_in: Option<Vec<u32>>,

    /// tensor dimensions out (optional)
    #[serde(default)]
    pub tensor_dimensions_out: Option<Vec<u32>>,
}

impl ModelMetadata {
    /// load metadata from json
    pub fn from_json(data: &[u8]) -> anyhow::Result<Self> {
        serde_json::from_slice(data).map_err(|e| anyhow!(format!("invalid json: {}", e)))
    }
}

pub struct ModelLoader {}

impl ModelLoader {
    /// get model and metadata
    pub async fn get_model_and_metadata(
        metadata_path: &str,
        model_path: &str,
    ) -> anyhow::Result<(ModelMetadata, Vec<u8>)> {
        let metadata_url = format!("https://ml-demo.cosmonic.app/{metadata_path}");
        let model_url = format!("https://ml-demo.cosmonic.app/{model_path}");

        let metadata_blob = match reqwest::get(metadata_url).await {
            Ok(resp) => resp
                .bytes()
                .await
                .map_err(|e| anyhow!("failed to parse metadata bytes: {e}"))?,
            Err(e) => bail!("Failed to request model metadata: {e:?}"),
        };

        // storing metadata makes sense when model data is done
        let metadata: ModelMetadata = ModelMetadata::from_json(&metadata_blob)
            .map_err(|e| anyhow!("ParsingMetadataError: {e}"))?;

        let model_name = metadata
            .model_name
            .clone()
            .unwrap_or_else(|| "UNKNOWN".to_string());

        log::info!(
            "successfully downloaded metadata '{}' of size {}",
            model_name,
            metadata_blob.len()
        );

        if !crate::model_encoding_enabled(metadata.graph_encoding) {
            bail!(
                "ModelNotEnabledError: Skipping model {}: graph type {:?} not enabled",
                model_name,
                metadata.graph_encoding
            );
        }

        let model_data_blob: Vec<u8> = reqwest::get(model_url)
            .await
            .map_err(|e| anyhow!("failed to fetch model '{model_name}': {e}"))?
            .bytes()
            .await
            .map_err(|e| anyhow!("failed to fetch model '{model_name}': {e}"))?
            .to_vec();
        log::info!(
            "successfully downloaded model '{}' of size {}",
            model_name,
            model_data_blob.len()
        );

        Ok((metadata, model_data_blob))
    }
}
