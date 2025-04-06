#!/bin/bash

# Run the MySQL Docker container
docker run -d --name mysql_container -e MYSQL_ROOT_PASSWORD=root_password -e MYSQL_DATABASE=url_shortening_service -p 3306:3306 mysql_image
