#!/bin/bash
# Author: michael.lichtenecker@stackblog.io

# this bash script simply reads the version specified in cargo.toml and sets it into the docker build command.
# after successfully building the image it will be pushed to the docker-dev repo.

COUNTER=0
APP_VERSION=0
IFS="="

# reads the config file
while read -r name value
do
  if [[ $COUNTER = "2" ]]; then
    APP_VERSION=$value
    break
  fi
  COUNTER=$((COUNTER+1))
done < "Cargo.toml"

# parses the version, removes the string marks
version=${APP_VERSION:2:-2}
echo $version

# builds the image and pushes it
#docker build -t docker-dev.stackblog.io:5000/opensight_analytics_api:$version .

# docker login, for travis CI
echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USERNAME" --password-stdin

#docker push docker-dev.stackblog.io:5000/opensight_analytics_api:$version