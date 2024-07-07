# Keter Rest

A Rust Axum RESTful API template.

- Basic CRUD RESTful API
- Request validation
- Authentication & Authorization
- Database connection & migration
- Custom Extractor
- File Uploading

## Getting started

Start PostgreSQL DB from docker

```shell
docker compose -d up
```

Create .env file from .env.example, then fill the necessary environment variables

```shell
cp .env.example .env
```

Run database migrations

```shell
# Install sqlx-cli (Optional) - dont run if already installed
cargo install sqlx-cli

# Construct DATABASE_URL from .env file
export $(<.env grep -v "^#" | xargs)
export DATABASE_URL="postgresql://$PG_DATABASE_HOST:$PG_DATABASE_PORT/$PG_DATABASE_DB?user=$PG_DATABASE_USERNAME&password=$PG_DATABASE_PASSWORD"

# Run migrations
sqlx migrate run
```

Start API process

```shell
cargo run
```

## Development

### Docker dependencies

```shell
# All docker dependencies are listed in docker-compose.yml
# Start docker compose
docker compose up -d

# Stop docker compose
docker compose down
```

### Database Migrations

```shell
# Install sqlx-cli before run
cargo install sqlx-cli

# Create reversible migration (-r flag)
sqlx migrate add -r "<name>"

# Run migrations
sqlx migrate run

# Run migrations
sqlx migrate revert
```

### API development

```shell
# Run api process in development mode
cargo run

# Build source code for deployment
cargo build --release

# Build docker image
docker build . -t keter-rest:v1

# Start container services
docker compose -f staging.docker-compose.yml up -d

# Stop container services
docker compose -f staging.docker-compose.yml down
```

## License

This project is under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0)
