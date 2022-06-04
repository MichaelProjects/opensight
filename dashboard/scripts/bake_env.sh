#!/bin/bash

#env_path="/usr/share/nginx/html/assets/assets/.env"
env_path="/Users/michael/Documents/Analytics/dashboard/.env"
rm $env_path
touch $env_path

if [ -z "$OPENSIGHT_CORE_URL" ];
  then echo "$OPENSIGHT_CORE_URL is null";
  else echo "OPENSIGHT_CORE_URL=$OPENSIGHT_CORE_URL" >> $env_path;
fi
