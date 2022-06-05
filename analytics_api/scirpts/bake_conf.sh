#!/bin/bash

filename="conf.toml"
conf_url="https://github.com/MichaelProjects/opensight/blob/dev/analytics/example_conf.toml"

if [[ -f $filename ]]
    then echo "$filename exists on your filesystem."
    else wget -O $filename $conf_url 
fi

exec "$@"