#!/usr/bin/env bash

for path in $(ls ./pass); do
	cd ./pass/$path && make clean && cd ../../
done;
