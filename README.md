# url-shorting-service-rust

This project is a URL shortening service built in Rust using Actix-web and SQLx.

## How to start the project

1. Clone this repository:

   ```bash
   git clone https://github.com/your-username/url-shorting-service-rust.git
   ```

2. Navigate to the project directory:

   ```bash
   cd url-shorting-service-rust
   ```

3. Create a `.env` file in the root of the project with your database configuration:

   ```env
   DATABASE_URL=mysql://user:password@localhost/database_name
   ```

4. Build and run the project:

   ```bash
   cargo run
   ```

5. Ensure Docker is installed and run the script to build the MySQL image:

   ```bash
   ./docker/run_mysql_build.sh
   ```

6. The server will be available at `http://127.0.0.1:8080`.

## Available Endpoints

### Create a shortened URL

```bash
curl -X POST http://localhost:8080/shorten -H "Content-Type: application/json" -d '{"url":"https://www.example.com"}'
```

### Retrieve a shortened URL

```bash
curl -X GET http://localhost:8080/shorten/{short}
```

### Update a shortened URL

```bash
curl -X PUT http://localhost:8080/shorten/{short} -H "Content-Type: application/json" -d '{"url":"https://www.updated-example.com"}'
```

### Delete a shortened URL

```bash
curl -X DELETE http://localhost:8080/shorten/{short}
```

### Get statistics for a shortened URL

```bash
curl -X GET http://localhost:8080/shorten/{short}/stats
```

### Check service health

```bash
curl -X GET http://localhost:8080/health
```

## Additional Resources

Check the project roadmap at the following link:
[Roadmap for the URL Shortening Service](https://roadmap.sh/projects/url-shortening-service)
