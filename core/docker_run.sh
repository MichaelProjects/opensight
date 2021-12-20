docker run \
 --mount type=bind,source="$(pwd)"/conf.toml,target=/conf.toml \
 -p 0.0.0.0:28018:28018 \
 -d \
 --network=analytics_cosmic_network \
 docker-dev.stackblog.io:5000/opensight_core:0.1.0