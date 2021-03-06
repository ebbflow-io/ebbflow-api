#!/bin/bash
set -e

java -jar ~/openapi-generator-cli.jar generate -i openapi/v1.yml -g rust-server -o openapi-build

cp openapi-build/src/models.rs src/generatedmodels.rs

sed -i '/use models;/d' ./src/generatedmodels.rs
