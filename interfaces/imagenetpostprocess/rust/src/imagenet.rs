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

/// Classification
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Classification {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub probability: f32,
}

// Encode Classification as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_classification<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Classification,
) -> RpcResult<()> {
    e.array(2)?;
    e.str(&val.label)?;
    e.f32(val.probability)?;
    Ok(())
}

// Decode Classification from cbor input stream
#[doc(hidden)]
pub fn decode_classification(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<Classification, RpcError> {
    let __result = {
        let mut label: Option<String> = None;
        let mut probability: Option<f32> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Classification, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => label = Some(d.str()?.to_string()),
                    1 => probability = Some(d.f32()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "label" => label = Some(d.str()?.to_string()),
                    "probability" => probability = Some(d.f32()?),
                    _ => d.skip()?,
                }
            }
        }
        Classification {
            label: if let Some(__x) = label {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Classification.label (#0)".to_string(),
                ));
            },

            probability: if let Some(__x) = probability {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Classification.probability (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
pub type Matches = Vec<Classification>;

// Encode Matches as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_matches<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Matches,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_classification(e, item)?;
    }
    Ok(())
}

// Decode Matches from cbor input stream
#[doc(hidden)]
pub fn decode_matches(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Matches, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<Classification> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(decode_classification(d).map_err(|e| {
                    format!(
                        "decoding 'org.wasmcloud.interface.mlimagenet#Classification': {}",
                        e
                    )
                })?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<Classification> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(decode_classification(d).map_err(|e| {
                        format!(
                            "decoding 'org.wasmcloud.interface.mlimagenet#Classification': {}",
                            e
                        )
                    })?),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// Description of Imagenet service
/// wasmbus.actorReceive
#[async_trait]
pub trait Imagenet {
    /// Converts the input string to a result
    async fn postprocess(
        &self,
        ctx: &Context,
        arg: &wasmcloud_interface_mlinference::InferenceOutput,
    ) -> RpcResult<Matches>;
}

/// ImagenetReceiver receives messages defined in the Imagenet service trait
/// Description of Imagenet service
#[doc(hidden)]
#[async_trait]
pub trait ImagenetReceiver: MessageDispatch + Imagenet {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "Postprocess" => {
                let value: wasmcloud_interface_mlinference::InferenceOutput =
                    wasmbus_rpc::common::decode(
                        &message.arg,
                        &wasmcloud_interface_mlinference::decode_inference_output,
                    )
                    .map_err(|e| RpcError::Deser(format!("'InferenceOutput': {}", e)))?;
                let resp = Imagenet::postprocess(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_matches(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "Imagenet.Postprocess",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Imagenet::{}",
                message.method
            ))),
        }
    }
}

/// ImagenetSender sends messages to a Imagenet service
/// Description of Imagenet service
/// client for sending Imagenet messages
#[derive(Debug)]
pub struct ImagenetSender<T: Transport> {
    transport: T,
}

impl<T: Transport> ImagenetSender<T> {
    /// Constructs a ImagenetSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> ImagenetSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl ImagenetSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Imagenet for ImagenetSender<T> {
    #[allow(unused)]
    /// Converts the input string to a result
    async fn postprocess(
        &self,
        ctx: &Context,
        arg: &wasmcloud_interface_mlinference::InferenceOutput,
    ) -> RpcResult<Matches> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        wasmcloud_interface_mlinference::encode_inference_output(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Imagenet.Postprocess",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: Matches = wasmbus_rpc::common::decode(&resp, &decode_matches)
            .map_err(|e| RpcError::Deser(format!("'{}': Matches", e)))?;
        Ok(value)
    }
}
