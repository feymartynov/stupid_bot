#!/bin/bash

docker build -t stupid_bot_build .
docker run --rm -v $(pwd):/build stupid_bot_build
