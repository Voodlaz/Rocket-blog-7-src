web: ROCKET_PORT=$PORT ROCKET_ENV=prod ./target/release/prettycode_blog ROCKET_DATABASES='{prettycode_blog_db={url="'"$DATABASE_URL"'"}}
release: ./target/release/diesel migration run
