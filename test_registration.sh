#!/bin/bash

echo "Testing Medicine Manager Registration Endpoint"
echo "=============================================="

# Start the backend in the background
cd /Users/ts/code/medman
echo "Starting backend..."
cargo run &
BACKEND_PID=$!

# Wait a bit for the server to start
sleep 5

# Test the registration endpoint
echo "Testing registration endpoint..."
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "testuser", "password": "testpass"}' \
  -v

# Clean up
kill $BACKEND_PID
wait $BACKEND_PID 2>/dev/null

echo "Test completed"
