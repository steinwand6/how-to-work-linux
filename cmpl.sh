#!/usr/bin/bash 
rustc $1 -o $(echo $1 | sed -e "s/rs/out/g")
