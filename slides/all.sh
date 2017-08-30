#!/bin/bash
trap "exit" INT TERM
trap "kill 0" EXIT
./run_server.sh &
./host.sh &
./loop Chrome &
wait
