web: ROCKET_PORT=$PORT ROCKET_ENV=prod ROCKET_DATABASES='{prettycode_blog_db={url="'"$DATABASE_URL"'"}}' ./target/release/prettycode_blog
release: ./target/release/diesel migration run
