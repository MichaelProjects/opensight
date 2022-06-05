#!/bin/bash

filename="conf.toml"
conf_url="https://raw.githubusercontent.com/MichaelProjects/opensight/dev/core/example_conf.toml"

if [[ -f $filename ]]
    then echo "$filename exists on your filesystem."
    else wget -O "$filename" $conf_url 
fi

exec "$@"