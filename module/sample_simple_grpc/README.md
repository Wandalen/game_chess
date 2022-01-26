# Simple and short gRPC explanation

gRPC - is a high-performance RPC (Remote Procedure call) framework created by Google. It runs on top of HTTP2.

## What is gRPC

gRPC is another one way to communicate between server and client over the network. The goal of the GRPC to make this communication more simple, more convenient.
All communication looks like simple function call:

```rust
  let move_result = chess_client.make_move(game_move).await.unwrap();
```

Under the hood `make_move` will make a request to the gRPC server and returns response from it.

## How it knows about requests and response formats?

We describe them in `.proto` file. Example of the simple `.proto` file:

```protobuf
syntax = "proto3";

package chess_example;

service ChessState {
  rpc make_move (GameMoveRequest) returns (MoveResultResponse);
}

message GameMoveRequest {
  int32 figure_id = 1;
  Position to = 2;
}

message MoveResultResponse {
  int32 status = 1;
  string message = 2;
}

message Position {
  int32 row = 1;
  int32 column = 2;
}
```

I think its syntax is self-explained.

## General workflow

1) We write a `.proto` file
2) Then with `protobuff` we generate code for our requests, responses, services, etc
3) We use generated code in our projects :)
4) Stay happy and healthy
