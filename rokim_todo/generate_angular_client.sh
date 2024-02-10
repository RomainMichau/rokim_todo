#!/bin/bash

# Start Rust app
echo "Starting Rust app..."
cargo run &
bg_pid=$!

# Wait for the Rust app to start (adjust the sleep time based on your app's startup time)
sleep 5

# Make a call to the OpenAPI documentation endpoint and save the response
echo "Fetching OpenAPI documentation..."
API_DOCS=$(curl -s http://localhost:8080/api-docs/openapi.json)

# Generate Angular client using OpenAPI Generator
echo "Generating Angular client..."
rm -rf ../rokim_todo_ui/src/app/services/todo
npx @openapitools/openapi-generator-cli generate -i <(echo $API_DOCS) -g typescript-angular -o ../rokim_todo_ui/src/app/services/todo

# Stop Rust app (assuming your Rust app has a shutdown endpoint)
echo "Stopping Rust app..."
kill "$bg_pid"

# Wait for the Rust app to stop (adjust the sleep time based on your app's shutdown time)
sleep 10

echo "Script completed."