docker run \
 --mount type=bind,source="$(pwd)"/config.toml,target=/config.toml \
 -p 0.0.0.0:28018:28018 \
 docker-dev.stackblog.io:5000/analytics_api:0.3