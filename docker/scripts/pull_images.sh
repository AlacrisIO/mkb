#!/usr/bin/env bash

# Build prerequisites image used for building side chain manager and client
echo "Pulling prerequisites images....please wait, this might take a while depending on your Internet connection speed"

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

for i in alacris_mkb_build_prerequisites:v1 alacris_mkb_run_prerequisites:v1;
  do
run docker pull gcr.io/legicash-demo-1950/legicash-demo/$i;
done
