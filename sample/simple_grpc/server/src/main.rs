use tonic::{Request, Response, Status};
use tonic::transport::Server;

#[allow(unused_imports)]
use sample_server::generated::chess_example::{GameMoveRequest, Position, MoveResultResponse};
use sample_server::generated::chess_example::chess_state_server::{ChessState, ChessStateServer};

struct ChessStateManager;

impl ChessStateManager {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl ChessState for ChessStateManager {
    async fn make_move(&self, _request: Request<GameMoveRequest>) -> Result<Response<MoveResultResponse>, Status> {
        Ok(Response::new(MoveResultResponse {
            status: 0,
            message: "success".to_owned(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Simple grpc server");

    let chess_state_manager = ChessStateManager::new();

    let addr = "[::1]:50051".parse()?;
    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(ChessStateServer::new(chess_state_manager))
        .serve(addr)
        .await?;
    Ok(())
}
