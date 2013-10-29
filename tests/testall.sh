#!/usr/bin/env bash

for path in $(ls ./pass); do
	cd ./pass/$path && make build && make test && cd ../../
	./compare.py $path
done;
