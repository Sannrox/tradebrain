use tradebrain::server::MyValueInvestingService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = MyValueInvestingService::new();

    service.run_server().await?;

    Ok(())
}
