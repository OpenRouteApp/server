use tonic::{Request, Response, Status, transport::Server};
use tonic_web::GrpcWebLayer;
use uuid::Uuid;

use prost_types::Timestamp;
use route::route_service_server::{RouteService, RouteServiceServer};
use route::{
    Coordinate, CreateRouteRequest, CreateRouteResponse, GetRouteRequest, GetRouteResponse, Route,
};

pub mod route {
    tonic::include_proto!("orroutes");
}

#[derive(Default)]
pub struct RouteManager {}

#[tonic::async_trait]
impl RouteService for RouteManager {
    async fn create_route(
        &self,
        _request: Request<CreateRouteRequest>,
    ) -> Result<Response<CreateRouteResponse>, Status> {
        let new_uuid = Uuid::new_v4();

        Ok(Response::new(CreateRouteResponse {
            route_id: new_uuid.to_string(),
        }))
    }

    async fn get_route(
        &self,
        request: Request<GetRouteRequest>,
    ) -> Result<Response<GetRouteResponse>, Status> {
        let route_id = request.into_inner().route_id;

        // Placeholder
        let route = Route {
            route_id,
            start: Some(Coordinate {
                latitude: 3.0,
                longitude: 3.0,
            }),
            end: Some(Coordinate {
                latitude: 3.0,
                longitude: 3.0,
            }),
            departure_time: Some(Timestamp {
                seconds: 1_687_000_000,
                nanos: 0,
            }),
        };

        Ok(Response::new(GetRouteResponse { route: Some(route) }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let route_service = RouteManager::default();

    println!("RouteServiceServer listening on {}", addr);

    Server::builder()
        .layer(GrpcWebLayer::new())
        .add_service(RouteServiceServer::new(route_service))
        .serve(addr)
        .await?;

    Ok(())
}
