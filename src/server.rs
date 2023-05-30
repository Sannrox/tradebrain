use tonic::{transport::Server, Request, Response, Status};
use valueinvesting::value_investing_service_server::{
    ValueInvestingService, ValueInvestingServiceServer,
};
use valueinvesting::{
    BookValuePerShareRequest, BookValuePerShareResponse, CompanyData, EquityRequest,
    EquityResponse, GrahamNumberRequest, GrahamNumberResponse, ValueInvestingMetrics,
};

pub mod valueinvesting {
    tonic::include_proto!("valueinvesting");
}

#[derive(Default, Clone)]
pub struct MyValueInvestingService {}

#[tonic::async_trait]
impl ValueInvestingService for MyValueInvestingService {
    async fn graham_number(
        &self,
        request: Request<GrahamNumberRequest>,
    ) -> Result<Response<GrahamNumberResponse>, Status> {
        let company_data = request.into_inner();
        // Calculation logic goes here...
        let metrics = GrahamNumberResponse { graham_number: 0.0 };

        Ok(Response::new(metrics))
    }
    async fn book_value_per_share(
        &self,
        request: Request<BookValuePerShareRequest>,
    ) -> Result<Response<BookValuePerShareResponse>, Status> {
        let company_data = request.into_inner();
        // Calculation logic goes here...
        let metrics = BookValuePerShareResponse {
            book_value_per_share: 0.0,
        };
        Ok(Response::new(metrics))
    }
    async fn calculate_shareholders_equity(
        &self,
        request: Request<EquityRequest>,
    ) -> Result<Response<EquityResponse>, Status> {
        let company_data = request.into_inner();
        // Calculation logic goes here...
        let metrics = EquityResponse {
            shareholders_equity: 0.0,
        };
        Ok(Response::new(metrics))
    }
    async fn calculate_metrics(
        &self,
        request: Request<CompanyData>,
    ) -> Result<Response<ValueInvestingMetrics>, Status> {
        let company_data = request.into_inner();
        // Calculation logic goes here...
        let metrics = ValueInvestingMetrics {
            pe_ratio: 0.0,
            pb_ratio: 0.0,
            // ...
        };
        Ok(Response::new(metrics))
    }
}

impl MyValueInvestingService {
    pub fn new() -> Self {
        MyValueInvestingService {}
    }

    pub fn default() -> Self {
        Self::new()
    }

    pub async fn run_server(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = "[::0]:50051".parse().unwrap();
        let service = ValueInvestingServiceServer::new(self.clone());

        println!("Value investing server listening on {}", addr);
        Server::builder().add_service(service).serve(addr).await?;

        Ok(())
    }
}
