use tonic::{transport::Server, Request, Response, Status};

use metrics::metrics_service_server::{MetricsService,MetricsServiceServer};
use metrics::{ExportMetricsServiceResponse, ExportMetricsServiceRequest};

pub mod metrics {
    tonic::include_proto!("metrics");
}

#[derive(Default)]
pub struct MyMetrics {}

#[tonic::async_trait]
impl MetricsService for MyMetrics {
    async fn export(
        &self,
        request: Request<ExportMetricsServiceRequest>,
    ) -> Result<Response<ExportMetricsServiceResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        println!("request data ==> {:?}", request);


        let reply = metrics::ExportMetricsServiceResponse {};
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50057".parse().unwrap();
    let greeter = MyMetrics::default();

    println!("OTLPServer listening on {}", addr);

    Server::builder()
        .add_service(MetricsServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
