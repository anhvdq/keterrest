# Rust Axum Framework - RESTful API Template

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
