use super::ExportRpc;
use stubs::export::v0 as export_grpc;
use stubs::export::v0::export_service_server::ExportService;
use tonic::{Request, Response, Status};

type ExportResult<T> = Result<T, Status>;

type EvalRequest = Request<export_grpc::EvalRequest>;
type EvalResponse = Response<export_grpc::EvalResponse>;
type EvalResult = ExportResult<EvalResponse>;

type ListIndicationRequest = Request<export_grpc::ListIndicationRequest>;
type ListIndicationResponse = Response<export_grpc::ListIndicationResponse>;
type ListIndicationResult = ExportResult<ListIndicationResponse>;

type GetArgumentValueRequest = Request<export_grpc::GetArgumentValueRequest>;
type GetArgumentValueResponse = Response<export_grpc::GetArgumentValueResponse>;
type GetArgumentValueResult = ExportResult<GetArgumentValueResponse>;

type PerformClickableActionRequest = Request<export_grpc::PerformClickableActionRequest>;
type PerformClickableActionResponse = Response<export_grpc::PerformClickableActionResponse>;
type PerformClickableActionResult = ExportResult<PerformClickableActionResponse>;

type GetSelfDataRequest = Request<export_grpc::Empty>;
type GetSelfDataResponse = Response<export_grpc::GetSelfDataResponse>;
type GetSelfDataResult = ExportResult<GetSelfDataResponse>;

#[tonic::async_trait]
impl ExportService for ExportRpc {
    async fn eval(&self, request: EvalRequest) -> EvalResult {
        if !self.eval_enabled {
            return Err(Status::permission_denied("eval operation is disabled"));
        }
        let json: String = self.request("exportEval", request).await?;
        Ok(Response::new(export_grpc::EvalResponse { json }))
    }

    async fn list_indication(&self, request: ListIndicationRequest) -> ListIndicationResult {
        let result = self.request("listIndication", request).await?;
        Ok(Response::new(result))
    }

    async fn get_argument_value(&self, request: GetArgumentValueRequest) -> GetArgumentValueResult {
        let result = self.request("getArgumentValue", request).await?;
        Ok(Response::new(result))
    }

    async fn perform_clickable_action(
        &self,
        request: PerformClickableActionRequest,
    ) -> PerformClickableActionResult {
        let result = self.request("performClickableAction", request).await?;
        Ok(Response::new(result))
    }

    async fn get_self_data(&self, request: GetSelfDataRequest) -> GetSelfDataResult {
        let json: String = self.request("getSelfData", request).await?;
        Ok(Response::new(export_grpc::GetSelfDataResponse { json }))
    }
}
