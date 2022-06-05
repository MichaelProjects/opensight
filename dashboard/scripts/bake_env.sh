#!/bin/bash
# This scirpt is used to create a .env file with the values that where specified in the -e of the docker run command.

env_path="/var/www/dashboard/assets/.env"
#env_path="/Users/michael/Documents/Analytics/dashboard/.env"
touch $env_path

if [ -z "$OPENSIGHT_CORE_URL" ];
  then echo "$OPENSIGHT_CORE_URL is null";
  else echo "OPENSIGHT_CORE_URL=$OPENSIGHT_CORE_URL" >> $env_path;
fi


exec "$@"