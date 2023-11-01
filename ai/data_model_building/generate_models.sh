#!/bin/bash

BASEDIR=$(dirname "$0")

GENERATOR_FILE="army_builder.py"

REPOSITORY_NAME="T9A-Records"
REPOSITORY_URL="git@github.com:Prophecy-eip/T9A-Records.git"

DATA_FILE="data.json"

clone_repository() {
    git clone ${REPOSITORY_URL}
}

generate_models() {
    python3 ${BASEDIR}/${GENERATOR_FILE} ${REPOSITORY_NAME}/${DATA_FILE}
}

clone_repository
generate_models
