#!/bin/bash

set -x 

solana-test-validator -r > v.log 2>&1 &

last_pid=$!
sleep 2

(
    cd native-2048
    cargo build-bpf
    solana program deploy 
    solana program deploy target/deploy/native_2048.so
)

sleep 1

(
    cd solana-operator
    yarn test 
)

kill -9 $last_pid
