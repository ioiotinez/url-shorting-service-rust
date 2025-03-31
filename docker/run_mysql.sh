#!/bin/bash
# filepath: /home/ioiotinez/dev/github/url-shorting-service-rust/docker/run_mysql.sh

# Build the Docker image
echo "Building the Docker image..."
docker build -t url-shortening-mysql -f Dockerfile .

# Run the Docker container
echo "Starting the MySQL container..."
docker run --rm -d -p 3306:3306 --name url-shortening-db url-shortening-mysql

echo "MySQL container is running on port 3306."