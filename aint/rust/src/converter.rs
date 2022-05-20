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

/// Description of Converter service
/// wasmbus.actorReceive
#[async_trait]
pub trait Converter {
    /// Converts the input string to a result
    async fn convert<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<String>;
}

/// ConverterReceiver receives messages defined in the Converter service trait
/// Description of Converter service
#[doc(hidden)]
#[async_trait]
pub trait ConverterReceiver: MessageDispatch + Converter {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "Convert" => {
                let value: String = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'String': {}", e)))?;
                let resp = Converter::convert(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Converter.Convert",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Converter::{}",
                message.method
            ))),
        }
    }
}

/// ConverterSender sends messages to a Converter service
/// Description of Converter service
/// client for sending Converter messages
#[derive(Debug)]
pub struct ConverterSender<T: Transport> {
    transport: T,
}

impl<T: Transport> ConverterSender<T> {
    /// Constructs a ConverterSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> ConverterSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl ConverterSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Converter for ConverterSender<T> {
    #[allow(unused)]
    /// Converts the input string to a result
    async fn convert<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<String> {
        let buf = wasmbus_rpc::common::serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Converter.Convert",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: String = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': String", e)))?;
        Ok(value)
    }
}
