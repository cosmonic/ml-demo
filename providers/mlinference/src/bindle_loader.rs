//use crate::{BindlePath};
use crate::{ExecutionTarget, GraphEncoding};
use bindle::client::{tokens::NoToken, Client};
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

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
    pub fn from_json(data: &[u8]) -> Result<Self, Error> {
        serde_json::from_slice(data)
            .map_err(|e| Error::InvalidParameter(format!("invalid json: {}", e)))
    }
}

/// errors generated by this crate
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("invalid parameter: {0}")]
    InvalidParameter(String),
}

pub struct BindleLoader {}

impl BindleLoader {
    /// provide
    pub async fn provide(bindle_url: &str) -> BindleResult<Client<NoToken>> {
        // init the connection to bindle
        let url = std::env::var(bindle_url).map_err(|_| BindleError::NoBindleUrlDefinedError)?;

        let bindle_client =
            Client::new(&url, NoToken).map_err(|_| BindleError::BindleUrlInvalidError)?;

        Ok(bindle_client)
    }

    /// get model and metadata
    pub async fn get_model_and_metadata(
        bindle_client: &Client<NoToken>,
        bindle_url: &str,
    ) -> BindleResult<(ModelMetadata, Vec<u8>)> {
        let invoice = bindle_client
            .get_invoice(bindle_url)
            .await
            .map_err(|_| BindleError::BindleInvoiceNotFoundError(bindle_url.to_string()))?;

        let parcels = invoice
            .parcel
            .ok_or_else(|| BindleError::BindleParcelNotFoundError(bindle_url.to_string()))?;

        let model_parcel = BindleLoader::get_first_member_of(&parcels, "model")
            .map_err(|_| BindleError::BindleNoParcelOfGroupModelError)?;

        let metadata_parcel = BindleLoader::get_first_member_of(&parcels, "metadata")
            .map_err(|_| BindleError::BindleNoParcelOfGroupMetadataError)?;

        let model_data_blob: Vec<u8> = bindle_client
            .get_parcel(bindle_url, &model_parcel.label.sha256)
            .await
            .map_err(|_| {
                BindleError::BindleParcelNotFetchedError(model_parcel.label.name.to_string())
            })?;
        log::info!(
            "successfully downloaded model {} of size {}",
            model_parcel.label.name,
            model_data_blob.len()
        );

        let metadata_blob: Vec<u8> = bindle_client
            .get_parcel(bindle_url, &metadata_parcel.label.sha256)
            .await
            .map_err(|_| {
                BindleError::BindleParcelNotFetchedError(metadata_parcel.label.name.to_string())
            })?;
        log::info!(
            "successfully downloaded metadata '{}' of size {}",
            metadata_parcel.label.name,
            metadata_blob.len()
        );

        // storing metadata makes sense when model data is done
        let metadata: ModelMetadata = ModelMetadata::from_json(&metadata_blob)
            .map_err(|error| {
                log::error!("BindleParsingMetadataError: '{}'", error);
                BindleError::BindleParsingMetadataError(format!("{}", error))
            })?;

        Ok((metadata, model_data_blob))
    }

    /// get first member of
    //fn get_first_member_of(parcels: &Vec<bindle::Parcel>, group: &str) -> BindleResult<&bindle::Parcel> {
    fn get_first_member_of<'a>(
        parcels: &'a [bindle::Parcel],
        group: &'a str,
    ) -> BindleResult<&'a bindle::Parcel> {
        let members = parcels
            .iter()
            .filter(|parcel| {
                parcel.conditions.is_some()
                    && parcel.conditions.as_ref().unwrap().member_of.is_some()
            })
            .filter(|parcel| {
                parcel
                    .conditions
                    .clone()
                    .unwrap()
                    .member_of
                    .unwrap()
                    .iter()
                    .any(|mbs| *mbs == group)
            })
            .collect::<Vec<&bindle::Parcel>>();

        if members.is_empty() {
            return Err(BindleError::BindleNoParcelOfGroupModelError);
        };

        members
            .into_iter()
            .next()
            .ok_or(BindleError::BindleNoParcelOfGroupModelError)
    }
}

/// BindleResult
pub type BindleResult<T> = Result<T, BindleError>;

#[derive(Debug, thiserror::Error)]
pub enum BindleError {
    #[error("No 'BINDLE_URL' defined, verify your bindle url.")]
    NoBindleUrlDefinedError,

    #[error("'BINDLE_URL' invalid, verify your bindle url.")]
    BindleUrlInvalidError,

    #[error("invoice {0} was not found on bindle server")]
    BindleInvoiceNotFoundError(String),

    #[error("parcel {0} was not found on bindle server")]
    BindleParcelNotFoundError(String),

    #[error("parcel {0} could not be downloaded from bindle server")]
    BindleParcelNotFetchedError(String),

    #[error("The invoice must have >0 parcels being member of group 'model'")]
    BindleNoParcelOfGroupModelError,

    #[error("The invoice must have >0 parcels being member of group 'metadata'")]
    BindleNoParcelOfGroupMetadataError,

    #[error("Error parsing metadata {0}")]
    BindleParsingMetadataError(String),
}
