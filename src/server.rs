use tonic::{transport::Server, Request, Response, Status};
use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments:: {BtcPaymentResponse, BtcPaymentRequest};

pub mod payments {
    tonic::include_proto!("payments");
}
#[derive(Debug, Default)]
pub struct BitcoinService {}

#[tonic::async_trait]
impl Bitcoin for BitcoinService {
    async fn sendpayment(
        &self,
        request: Request<BtcPaymentRequest>,     
    ) -> Result<Response<BtcPaymentResponse>, Status> {
        println!("Got a response: {:?}", request);

        let req = request.into_inner();

        let reply = BtcPaymentResponse {
            success: true,
            message: format!("sent {}BTC to {}.", req.amount, req.to_addr),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let btc_service = BitcoinService::default();

    Server::builder().add_service(BitcoinServer::new(btc_service)).serve(addr).await?;

    Ok(())

    
}
