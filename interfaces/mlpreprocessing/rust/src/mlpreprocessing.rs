// This file is generated automatically using wasmcloud/weld-codegen 0.4.3

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

/// ConversionOutput
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ConversionOutput {
    pub result: wasmcloud_interface_mlinference::Status,
    pub tensor: wasmcloud_interface_mlinference::Tensor,
}

// Encode ConversionOutput as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_conversion_output<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ConversionOutput,
) -> RpcResult<()> {
    e.array(2)?;
    wasmcloud_interface_mlinference::encode_status(e, &val.result)?;
    wasmcloud_interface_mlinference::encode_tensor(e, &val.tensor)?;
    Ok(())
}

// Decode ConversionOutput from cbor input stream
#[doc(hidden)]
pub fn decode_conversion_output(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ConversionOutput, RpcError> {
    let __result = {
        let mut result: Option<wasmcloud_interface_mlinference::Status> = None;
        let mut tensor: Option<wasmcloud_interface_mlinference::Tensor> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ConversionOutput, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        result = Some(wasmcloud_interface_mlinference::decode_status(d).map_err(
                            |e| {
                                format!(
                                    "decoding 'org.wasmcloud.interface.mlinference#Status': {}",
                                    e
                                )
                            },
                        )?)
                    }
                    1 => {
                        tensor = Some(wasmcloud_interface_mlinference::decode_tensor(d).map_err(
                            |e| {
                                format!(
                                    "decoding 'org.wasmcloud.interface.mlinference#Tensor': {}",
                                    e
                                )
                            },
                        )?)
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "result" => {
                        result = Some(wasmcloud_interface_mlinference::decode_status(d).map_err(
                            |e| {
                                format!(
                                    "decoding 'org.wasmcloud.interface.mlinference#Status': {}",
                                    e
                                )
                            },
                        )?)
                    }
                    "tensor" => {
                        tensor = Some(wasmcloud_interface_mlinference::decode_tensor(d).map_err(
                            |e| {
                                format!(
                                    "decoding 'org.wasmcloud.interface.mlinference#Tensor': {}",
                                    e
                                )
                            },
                        )?)
                    }
                    _ => d.skip()?,
                }
            }
        }
        ConversionOutput {
            result: if let Some(__x) = result {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ConversionOutput.result (#0)".to_string(),
                ));
            },

            tensor: if let Some(__x) = tensor {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ConversionOutput.tensor (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// ConversionRequest
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ConversionRequest {
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub data: Vec<u8>,
}

// Encode ConversionRequest as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_conversion_request<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ConversionRequest,
) -> RpcResult<()> {
    e.array(1)?;
    e.bytes(&val.data)?;
    Ok(())
}

// Decode ConversionRequest from cbor input stream
#[doc(hidden)]
pub fn decode_conversion_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ConversionRequest, RpcError> {
    let __result = {
        let mut data: Option<Vec<u8>> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ConversionRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => data = Some(d.bytes()?.to_vec()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "data" => data = Some(d.bytes()?.to_vec()),
                    _ => d.skip()?,
                }
            }
        }
        ConversionRequest {
            data: if let Some(__x) = data {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ConversionRequest.data (#0)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Error returned with InferenceOutput
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MlPError {
    /// n(0)
    RuntimeError(String),
    /// n(1)
    NotSupported(String),
}

// Encode MlPError as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_ml_p_error<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &MlPError,
) -> RpcResult<()> {
    // encoding union MlPError
    e.array(2)?;
    match val {
        MlPError::RuntimeError(v) => {
            e.u16(0)?;
            e.str(v)?;
        }
        MlPError::NotSupported(v) => {
            e.u16(1)?;
            e.str(v)?;
        }
    }
    Ok(())
}

// Decode MlPError from cbor input stream
#[doc(hidden)]
pub fn decode_ml_p_error(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<MlPError, RpcError> {
    let __result = {
        // decoding union MlPError
        let len = d.fixed_array()?;
        if len != 2 {
            return Err(RpcError::Deser(
                "decoding union 'MlPError': expected 2-array".to_string(),
            ));
        }
        match d.u16()? {
            0 => {
                let val = d.str()?.to_string();
                MlPError::RuntimeError(val)
            }

            1 => {
                let val = d.str()?.to_string();
                MlPError::NotSupported(val)
            }

            n => {
                return Err(RpcError::Deser(format!(
                    "invalid field number for union \
                     'org.wasmcloud.interface.mlpreprocessing#MlPError':{}",
                    n
                )));
            }
        }
    };
    Ok(__result)
}
/// Description of Mlpreprocessing service
/// wasmbus.actorReceive
#[async_trait]
pub trait MlPreprocessing {
    /// Converts the input string to a result
    async fn convert(&self, ctx: &Context, arg: &ConversionRequest) -> RpcResult<ConversionOutput>;
}

/// MlPreprocessingReceiver receives messages defined in the MlPreprocessing service trait
/// Description of Mlpreprocessing service
#[doc(hidden)]
#[async_trait]
pub trait MlPreprocessingReceiver: MessageDispatch + MlPreprocessing {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "Convert" => {
                let value: ConversionRequest =
                    wasmbus_rpc::common::decode(&message.arg, &decode_conversion_request)
                        .map_err(|e| RpcError::Deser(format!("'ConversionRequest': {}", e)))?;
                let resp = MlPreprocessing::convert(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_conversion_output(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "MlPreprocessing.Convert",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "MlPreprocessing::{}",
                message.method
            ))),
        }
    }
}

/// MlPreprocessingSender sends messages to a MlPreprocessing service
/// Description of Mlpreprocessing service
/// client for sending MlPreprocessing messages
#[derive(Debug)]
pub struct MlPreprocessingSender<T: Transport> {
    transport: T,
}

impl<T: Transport> MlPreprocessingSender<T> {
    /// Constructs a MlPreprocessingSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> MlPreprocessingSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl MlPreprocessingSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> MlPreprocessing
    for MlPreprocessingSender<T>
{
    #[allow(unused)]
    /// Converts the input string to a result
    async fn convert(&self, ctx: &Context, arg: &ConversionRequest) -> RpcResult<ConversionOutput> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_conversion_request(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "MlPreprocessing.Convert",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: ConversionOutput = wasmbus_rpc::common::decode(&resp, &decode_conversion_output)
            .map_err(|e| RpcError::Deser(format!("'{}': ConversionOutput", e)))?;
        Ok(value)
    }
}
