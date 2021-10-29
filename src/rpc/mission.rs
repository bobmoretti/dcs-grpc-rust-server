use std::pin::Pin;

use super::MissionRpc;
use crate::shutdown::AbortableStream;
use futures_util::{Stream, StreamExt};
use stubs::mission::mission_service_server::MissionService;
use stubs::timer::timer_service_server::TimerService;
use stubs::*;
use time::format_description::well_known::Rfc3339;
use time::{Date, Duration, Month, PrimitiveDateTime, Time, UtcOffset};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl MissionService for MissionRpc {
    type StreamEventsStream =
        Pin<Box<dyn Stream<Item = Result<mission::Event, tonic::Status>> + Send + Sync + 'static>>;
    type StreamUnitsStream = Pin<
        Box<dyn Stream<Item = Result<mission::UnitUpdate, tonic::Status>> + Send + Sync + 'static>,
    >;

    async fn stream_events(
        &self,
        _request: Request<mission::StreamEventsRequest>,
    ) -> Result<Response<Self::StreamEventsStream>, Status> {
        let events = self.events().await;
        let stream = AbortableStream::new(self.shutdown_signal.signal(), events.map(Ok));
        Ok(Response::new(Box::pin(stream)))
    }

    async fn stream_units(
        &self,
        request: Request<mission::StreamUnitsRequest>,
    ) -> Result<Response<Self::StreamUnitsStream>, Status> {
        let rpc = self.clone();
        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            if let Err(crate::stream::Error::Status(err)) =
                crate::stream::stream_units(request.into_inner(), rpc, tx.clone()).await
            {
                // ignore error, as we don't care at this point whether the channel is closed or not
                let _ = tx.send(Err(err)).await;
            }
        });

        let stream = AbortableStream::new(
            self.shutdown_signal.signal(),
            ReceiverStream::new(rx).map(|result| {
                result.map(|update| mission::UnitUpdate {
                    update: Some(update),
                })
            }),
        );
        Ok(Response::new(Box::pin(stream)))
    }

    async fn get_scenario_start_time(
        &self,
        _: Request<mission::GetScenarioStartTimeRequest>,
    ) -> Result<Response<mission::GetScenarioStartTimeResponse>, Status> {
        let start = self
            .get_time_zero(Request::new(timer::GetTimeZeroRequest {}))
            .await?
            .into_inner();

        Ok(Response::new(mission::GetScenarioStartTimeResponse {
            datetime: to_datetime(start.year, start.month, start.day, start.time)?,
        }))
    }

    async fn get_scenario_current_time(
        &self,
        _: Request<mission::GetScenarioCurrentTimeRequest>,
    ) -> Result<Response<mission::GetScenarioCurrentTimeResponse>, Status> {
        let current = self
            .get_absolute_time(Request::new(timer::GetAbsoluteTimeRequest {}))
            .await?
            .into_inner();

        Ok(Response::new(mission::GetScenarioCurrentTimeResponse {
            datetime: to_datetime(current.year, current.month, current.day, current.time)?,
        }))
    }
}

fn to_datetime(year: i32, month: u32, day: u32, time: f64) -> Result<String, Status> {
    let month = u8::try_from(month)
        .map_err(|err| Status::internal(format!("received invalid month: {}", err)))?;
    let month = Month::try_from(month)
        .map_err(|err| Status::internal(format!("received invalid month: {}", err)))?;
    let day = u8::try_from(day)
        .map_err(|err| Status::internal(format!("received invalid day: {}", err)))?;
    let date = Date::from_calendar_date(year, month, day)
        .map_err(|err| Status::internal(format!("received invalid date: {}", err)))?;
    let time = Time::from_hms(0, 0, 0).unwrap() + Duration::seconds(time as i64);
    let datetime = PrimitiveDateTime::new(date, time).assume_offset(UtcOffset::UTC);

    datetime.format(&Rfc3339).map_err(|err| {
        Status::internal(format!("failed to format date as ISO 8601 string: {}", err))
    })
}
