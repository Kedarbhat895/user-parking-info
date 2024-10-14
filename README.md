user-parking-info: A Rust Application for Parking Management

This project provides a simple Rust application for managing user parking information and reservations.
Features:

    Parking Reservation: Users can reserve parking spots and receive confirmation receipts.
    Spot Types: Three different types of parking spots are available, each with potentially varying rates.
    Payments: Users can pay for parking spots based on time spent. Different rates may apply based on the chosen spot type.
    Single Occupancy: Each parking spot can only accommodate one vehicle at a time.

Running the Application:

Requirements:

    Rust: Ensure you have Rust installed on your system. You can find installation instructions on the official Rust website (https://www.rust-lang.org/tools/install).
    Docker (Optional): If you want to use a local DynamoDB instance, Docker is recommended.

Setting Up Local DynamoDB (with Docker):

    Create a docker-compose.yml file:


version: "3.5"

services:
  dynamo:
    container_name: local-dynamodb
    image: amazon/dynamodb-local
    networks:
      - local-dynamodb
    ports:
      - "8000:8000"
    volumes:
      - dynamodata:/home/dynamodblocal
    working_dir: /home/dynamodblocal
    command: "-jar DynamoDBLocal.jar -sharedDb -dbPath ."

networks:
  local-dynamodb:
    name: local-dynamodb

volumes:
  dynamodata:  

 {}


To bring the dynamodb up run:
docker-compose up -d


Use the AWS CLI (if installed) to create a table and confirm your DynamoDB instance is running:
Bash

aws dynamodb create-table \
  --table-name user-info \
  --attribute-definitions AttributeName=email,AttributeType=S \
  --key-schema AttributeName=email,KeyType=HASH \
  --provisioned-throughput ReadCapacityUnits=1,WriteCapacityUnits=1  



Once Installed, you can build the project using cargo build and run it in debug mode using RUST_LOG=debug cargo run 