use serde_json::Value as JsonValue;
use std::string::String;
use stubs::export::v0 as export_rpc;
use stubs::export::v0::export_service_client::ExportServiceClient;
use stubs::hook::v0::hook_service_client::HookServiceClient;
use tonic;
use tonic::transport::Channel;

// use std::io;
use tokio::runtime as trt;
//, Code, Status};

#[derive(Debug, Clone)]
struct ServiceClients<T> {
    _hook: HookServiceClient<T>,
    export: ExportServiceClient<T>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Grpc(#[from] tonic::Status),
    #[error("failed to decode JSON result")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct DcsGrpcClient<T> {
    clients: ServiceClients<T>,
    runtime: trt::Runtime,
}

impl DcsGrpcClient<Channel> {
    pub fn list_indication(&mut self, device_id: i32) -> Result<String, Error> {
        let future = self
            .clients
            .export
            .list_indication(export_rpc::ListIndicationRequest { device_id });
        let result = self.runtime.block_on(future)?.into_inner();
        Ok(result.indication)
    }

    pub fn get_argument_value(
        &mut self,
        device_id: i32,
        argument_name: String,
    ) -> Result<f64, Error> {
        let request = export_rpc::GetArgumentValueRequest {
            device_id,
            argument_name,
        };
        let future = self.clients.export.get_argument_value(request);
        let result = self.runtime.block_on(future)?.into_inner();
        Ok(result.value)
    }

    pub fn perform_clickable_action(
        &mut self,
        device_id: i32,
        argument_name: String,
        value: f64,
    ) -> Result<(), Error> {
        let request = export_rpc::PerformClickableActionRequest {
            device_id,
            argument_name,
            value,
        };

        let future = self.clients.export.perform_clickable_action(request);
        self.runtime.block_on(future)?.into_inner();
        Ok(())
    }

    pub fn get_self_data(&mut self) -> Result<JsonValue, Error> {
        let request = export_rpc::Empty {};
        let future = self.clients.export.get_self_data(request);
        let result = self.runtime.block_on(future)?.into_inner();
        let json = serde_json::from_str(result.json.as_str())?;
        Ok(json)
    }

    pub fn get_aircraft_name(&mut self) -> Result<String, Error> {
        let json = self.get_self_data()?;
        return Ok(json["Name"].to_string());
    }

    pub fn new() -> Self {
        let runtime = trt::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_io()
            .build()
            .unwrap();

        let clients = runtime.block_on(start_clients()).unwrap();
        Self {
            clients: clients,
            runtime: runtime,
        }
    }
}

async fn start_clients() -> Result<ServiceClients<Channel>, Box<dyn std::error::Error>> {
    let hook_client = HookServiceClient::connect("http://127.0.0.1:50051").await?;
    let export_client = ExportServiceClient::connect("http://127.0.0.1:50051").await?;
    let clients = ServiceClients {
        _hook: hook_client,
        export: export_client,
    };
    Ok(clients)
}
