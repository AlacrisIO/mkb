#!/usr/bin/env bash
# Build prerequisites image used for building side chain manager and client
echo "Building prerequisites images....please wait, this might take a while depending on your Internet connection speed"

run() {
	$*
	if [ $? -ne 0 ]
	then
		echo "$* failed with exit code $?"
		return 1
	else
		return 0
	fi
}

echo "Building build prerequisite image"
run docker build -t gcr.io/legicash-demo-1950/legicash-demo/alacris_mkb_build_prerequisites:v1 -f docker/containers/alacris_mkb_build_prerequisites/Dockerfile .

echo "Building runtime prerequisites image"
run docker build -t gcr.io/legicash-demo-1950/legicash-demo/alacris_mkb_run_prerequisites:v1 -f docker/containers/alacris_mkb_run_prerequisites/Dockerfile .

echo "Building application images"
run docker-compose -f docker/docker-compose.yml build
