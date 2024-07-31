use greet::{
    greeter_server::{Greeter, GreeterServer},
    HelloReq, HelloResp,
};
use tonic::{transport::Server, Request, Response, Status};
use voting::{
    voting_server::{Voting, VotingServer},
    VotingRequest, VotingResponse,
};

pub mod voting {
    include!("../protos/voting.rs");
}
pub mod greet {
    include!("../protos/hello.rs");
}

#[derive(Debug, Default)]
pub struct VotingService {}
#[tonic::async_trait]
impl Voting for VotingService {
    async fn vote(
        &self,
        request: Request<VotingRequest>,
    ) -> Result<Response<VotingResponse>, Status> {
        let r: &VotingRequest = request.get_ref();
        match r.vote {
            0 => Ok(Response::new(voting::VotingResponse {
                confirmation: { format!("upvoted for {}", r.url) },
            })),
            1 => Ok(Response::new(voting::VotingResponse {
                confirmation: { format!("downvoted for {}", r.url) },
            })),
            _ => Err(Status::new(
                tonic::Code::OutOfRange,
                "Invalid vote provided",
            )),
        }
    }
}

#[derive(Debug)]
pub struct GreetService;
#[tonic::async_trait]
impl Greeter for GreetService {
    async fn say_hello(&self, request: Request<HelloReq>) -> Result<Response<HelloResp>, Status> {
        let hello_str = request.into_inner().content;
        println!("greet from client: {}", hello_str);
        Ok(Response::new(HelloResp { content: hello_str }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();
    let voting_service = VotingService::default();

    Server::builder()
        .add_service(VotingServer::new(voting_service))
        .add_service(GreeterServer::new(GreetService))
        .serve(address)
        .await?;
    Ok(())
}
