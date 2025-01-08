#!/bin/bash

function show_help() {
  echo -e "Invalid arguments ${0}\n\tUsage ./run.sh <path to file>"
}

if [[ $# == 0 ]]; then 
  show_help
  exit 1
fi

if [[ ! -f ${1} ]]; then
  echo "The PATH: '${1}' doesn't exist"
  exit 1
fi

basename=$(basename ${1}) #get file name
shopt -s nullglob #count number of files in src/bin
count=(*)
count=${#numfiles[@]}
file=$(echo "${1}" | sed -r "s/.+\/(.+)\..+/\1/") #get file name without the extension
if [[ -f src/bin/"${count}${basename}" ]]; then
  cat ${1} > src/bin/"${count}${basename}"
  RUSTFLAGS="--cfg DEBUG" cargo r --bin "${count}${file}"
else 
  ((count=count+1))
  cat ${1} > src/bin/"${count}${basename}"
  RUSTFLAGS="--cfg DEBUG" cargo r --bin "${count}${file}"
fi
