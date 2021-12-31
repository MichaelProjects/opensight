docker run \
 --mount type=bind,source="$(pwd)"/conf.toml,target=/conf.toml \
 -p 0.0.0.0:28019:28019 \
 -d \
 --network=opensight_net \
 docker-dev.stackblog.io:5000/opensight_core:0.1.0