#!/bin/bash

# Experiment auto creating a rust API from openapi formatted OBS documentation

podman pull openapitools/openapi-generator-cli
podman run --rm \
    -v /home/ms/Project/open-build-service:/local \
    openapitools/openapi-generator-cli \
    generate \
    -i /local/src/api/public/apidocs-new/paths/build.yaml \
    -g rust \
    -o /tmp/obs_api_build.rs


# java -jar /opt/openapi-generator/modules/openapi-generator-cli/target/openapi-generator-cli.jar generate -i /local/src/api/public/apidocs-new/paths/build.yaml -g rust -o /tmp/obs_api_build.rs
