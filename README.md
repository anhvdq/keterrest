# Rust Axum Framework - RESTful API Template

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

# Run migrations
sqlx migrate run
```

Start API process

```shell
cargo run
```

Test calling API

```shell
# Create a new user
curl --location 'localhost:3000/users' \
--header 'Content-Type: application/json' \
--data '{
    "name": "Test",
    "age": 10
}'
```

## Features

### Routing

### Scalability

### Logging

### Validation

### Error Handling

#### Paths

#### Form Requests

### Database Migrations

```shell
cargo install sqlx-cli
```

```shell
export $(<.env grep -v "^#" | xargs)
export DATABASE_URL="postgresql://$PG_DATABASE_HOST:$PG_DATABASE_PORT/$PG_DATABASE_DB?user=$PG_DATABASE_USERNAME&password=$PG_DATABASE_PASSWORD"
```

```shell
# Create reversible migration (-r flag)
sqlx migrate add -r "<name>"
```

## Development

### Testing

### External Services

### Deployment
