
use wasmbus_rpc::actor::prelude::*;
use aint::{Converter, ConverterReceiver};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Converter)]
struct ConvertingActor {}


#[async_trait]
impl Converter for ConvertingActor {

      /// Converts the input string to a result
      async fn convert<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        _ctx: &Context,
        arg: &TS,
    ) -> RpcResult<String> {
        let text = arg.to_string();
        Ok(text.to_ascii_uppercase())
    }
}

