#!/bin/bash

# Hashemi Login System Demo Script
# 🔐 Professional Login System with Rust + Axum

echo "� Starting Hashemi Login System Demo..."
echo "======================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Rust/Cargo is not installed. Please install Rust first:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Build the project
print_status "Building the project..."
if cargo build; then
    print_success "Build completed successfully!"
else
    print_error "Build failed. Please check the errors above."
    exit 1
fi

# Kill any existing processes on ports 3000 and 8080
print_status "Cleaning up existing processes..."
lsof -ti:3000 | xargs kill -9 2>/dev/null || true
lsof -ti:8080 | xargs kill -9 2>/dev/null || true

# Start the backend server
print_status "Starting backend server on port 3000..."
./target/debug/backend &
BACKEND_PID=$!

# Wait for backend to start
sleep 3

# Check if backend is running
if curl -s http://localhost:3000/health > /dev/null; then
    print_success "Backend server is running!"
else
    print_error "Backend server failed to start!"
    kill $BACKEND_PID 2>/dev/null || true
    exit 1
fi

# Start Leptos frontend server
print_status "Starting Leptos frontend server on port 8080..."
cd frontend
trunk serve --port 8080 --address 127.0.0.1 &
HTTP_PID=$!
cd ..

# Wait for HTTP server to start
sleep 2

# Test the API
print_status "Testing the login API..."
echo ""
echo "🧪 API Test Results:"
echo "==================="

echo "✅ Successful Login Test:"
curl -s -X POST http://localhost:3000/login \
    -H "Content-Type: application/json" \
    -d '{"email":"test@example.com","password":"test123"}' | python3 -m json.tool

echo ""
echo "❌ Failed Login Test:"
curl -s -X POST http://localhost:3000/login \
    -H "Content-Type: application/json" \
    -d '{"email":"test@example.com","password":"wrong"}' | python3 -m json.tool

echo ""
echo "🏥 Health Check:"
curl -s http://localhost:3000/health
echo ""

# Display demo information
echo ""
echo "🎉 Demo is now running!"
echo "======================"
echo ""
echo "📱 Leptos Frontend: http://localhost:8080"
echo "🔧 Backend API: http://localhost:3000"
echo "🔑 Demo Credentials:"
echo "   Email: test@example.com"
echo "   Password: test123"
echo ""
echo "📊 Available Endpoints:"
echo "   POST /login - Authenticate user"
echo "   GET  /health - Health check"
echo ""
echo "🛠️  Architecture: Onion Pattern"
echo "🦀 Backend: Rust + Axum"
echo "🎯 Frontend: Rust + Leptos"
echo "🔐 Auth: JWT + bcrypt"
echo ""

# Function to cleanup on exit
cleanup() {
    print_status "Shutting down servers..."
    kill $BACKEND_PID 2>/dev/null || true
    kill $HTTP_PID 2>/dev/null || true
    print_success "Demo stopped. Thank you!"
    exit 0
}

# Set trap to cleanup on script exit
trap cleanup SIGINT SIGTERM

# Keep the script running
print_warning "Press Ctrl+C to stop the demo"
while true; do
    sleep 1
done
