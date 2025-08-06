export $(<.env grep -v "^#" | xargs)
export DATABASE_URL="postgresql://$PG_DATABASE_HOST:$PG_DATABASE_PORT/$PG_DATABASE_DB?user=$PG_DATABASE_USERNAME&password=$PG_DATABASE_PASSWORD"

sqlx migrate run
