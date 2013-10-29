#!/usr/bin/env bash

for path in $(ls ./pass); do
	./compare.py $path
done;
