#!/bin/bash

set -euC

function show_help() {
  echo -e "Invalid Argument ${0}!\n\tUsage: ./makefile.sh <contest_folder> <contest_id> [<number_of_problems> / <problem>]"
  echo -e "\t<contest_folder>:\t{ex. cf, atcoder,...}"
  echo -e "\t<contest_id>:\t {ex. 1966, abc351}"
  echo -e "\t<number_of_problems> / <problem>:\t {ex. 1, 3} / {ex. A, C}"
}

if [[ $# < 2 ]]; then
  show_help
  exit 1
fi

#check if contest folder exists
if [[ ! -d "./src/${1}" ]]; then 
  mkdir "./src/${1}"  
fi

if [[ $# == 3 ]]; then
  #check if contest id exists
  if [[ ! -d "./src/${1}/${2}" ]]; then
    mkdir "./src/${1}/${2}"  
  fi
  
  reg='^[0-9]+([.][0-9]+)?$'
  
  if [[ ${3} =~ $reg ]]; then 
    #create problems files
    for (( i = 0 ; i < ${3}; i++ ));
    do
      ((n=i+97))
      fileId=$(printf "\\$(printf '%03o' "$n")")
      if [[ ! -f "./src/${1}/${2}/${fileId}.rs" ]]; then
        cat ./src/basic.rs >> ./src/${1}/${2}/${fileId}.rs
      fi
    done
  else 
    if [[ ! -f "./src/${1}/${2}/${3}.rs" ]]; then
      cat ./src/basic.rs >> ./src/${1}/${2}/${3}.rs
    fi
  fi
else 
  if [[ $# == 2 ]]; then
    if [[ ! -f "./src/${1}/${2}.rs" ]]; then
      cat ./src/basic.rs >> ./src/${1}/${2}.rs
    fi
  fi
fi
