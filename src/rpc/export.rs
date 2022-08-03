use super::ExportRpc;
use stubs::export::v0::export_service_server::ExportService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl ExportService for ExportRpc {
    async fn eval(
        &self,
        request: Request<export::v0::EvalRequest>,
    ) -> Result<Response<export::v0::EvalResponse>, Status> {
        if !self.eval_enabled {
            return Err(Status::permission_denied("eval operation is disabled"));
        }

        let json: String = self.request("exportEval", request).await?;
        Ok(Response::new(export::v0::EvalResponse { json }))
    }
}
