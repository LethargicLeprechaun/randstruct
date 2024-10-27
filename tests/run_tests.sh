#!/bin/bash

declare -a SEEDS=("e2f28820694c414d09646092a3bf9dec6cba6b389be38112ef808d56300f3d89"
                  "0000000000000000000000000000000000000000000000000000000000000000"
                  "6002212aa494e586420afe7d61da98041880ea903c816167bdc3a074f425c29b"
                  "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                  )

build_header() {
    echo "const char *randstruct_seed = \"$1\";" > randstruct.h
}

for SEED in ${SEEDS[@]}
do
    build_header ${SEED}
    RANDSTRUCT_SEED=${SEED} SEED_HEADER_FILE=`pwd`/randstruct.h cargo run
done
