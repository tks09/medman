#!/bin/bash

echo "🚀 Medicine Manager Setup Script"
echo "================================"
echo ""

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    echo "   Visit https://docs.docker.com/get-docker/ for installation instructions."
    exit 1
fi

# Check if docker-compose is installed
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose."
    echo "   It's usually included with Docker Desktop."
    exit 1
fi

echo "✅ Docker and Docker Compose are installed"
echo ""

# Copy environment file
if [ ! -f ".env" ]; then
    echo "📋 Copying environment file..."
    cp .env.example .env
    echo "✅ Created .env"
else
    echo "ℹ️  .env already exists"
fi

echo ""
echo "🔑 Please edit .env and set your MISTRAL_API_KEY"
echo "   You can get one from https://mistral.ai/"
echo ""

# Ask if user wants to proceed with Docker setup
read -p "🚀 Do you want to start the application with Docker? (y/n) " -n 1 -r
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo ""
    echo "🐳 Starting Docker containers..."
    docker-compose up -d

    if [ $? -eq 0 ]; then
        echo "✅ Containers started successfully!"
        echo ""
        echo "🎉 Application is now running:"
        echo "   - Frontend: http://localhost:5173"
        echo "   - Backend API: http://localhost:3000"
        echo "   - MongoDB UI: http://localhost:8081 (admin/admin123)"
        echo ""
        echo "📝 To stop the application, run: docker-compose down"
    else
        echo "❌ Failed to start containers. Check the error above."
    fi
else
    echo ""
    echo "ℹ️  Docker setup cancelled. You can start manually later with:"
    echo "   docker-compose up -d"
fi
