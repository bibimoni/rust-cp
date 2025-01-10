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
if [[ ! -d "/Users/distiled/codeStuff/rust/rustcp/src/${1}" ]]; then 
  mkdir "/Users/distiled/codeStuff/rust/rustcp/src/${1}"  
fi

if [[ $# == 3 ]]; then
  #check if contest id exists
  if [[ ! -d "/Users/distiled/codeStuff/rust/rustcp/src/${1}/${2}" ]]; then
    mkdir "/Users/distiled/codeStuff/rust/rustcp/src/${1}/${2}"  
  fi
  
  reg='^[0-9]+([.][0-9]+)?$'
  
  append_to_cargo_toml() {
    local rust_file_path="$1" 
    local cargo_toml_path="/Users/distiled/codeStuff/rust/rustcp/Cargo.toml"

    local problem="$(basename "$rust_file_path" .rs)"

    local contest_name="$(basename "$(dirname "$rust_file_path")")"
    local name="${contest_name}_${problem}"
    local bin_entry="[[bin]]\nname=\"$name\"\npath=\"$rust_file_path\"\n\n"

    if grep -q "path=\"$rust_file_path\"" "$cargo_toml_path"; then
      echo "Source file ${contest_name}/${problem}.rs has already been exists"
    else
      # Append the entry to Cargo.toml
      echo -e "$bin_entry" >> "$cargo_toml_path"
      echo "Source file ${contest_name}/${problem}.rs created"
    fi
  }

  if [[ ${3} =~ $reg ]]; then 
    # Create problem files
    for (( i = 0 ; i < ${3}; i++ ));
    do
      ((n=i+97))
      fileId=$(printf "\\$(printf '%03o' "$n")")
      target_path="/Users/distiled/codeStuff/rust/rustcp/src/${1}/${2}/${fileId}.rs"
      if [[ ! -f "$target_path" ]]; then
        cat /Users/distiled/codeStuff/rust/rustcp/src/basic.rs >> "$target_path"
        append_to_cargo_toml "$target_path" # Call the function here
      fi
    done
  else 
    target_path="/Users/distiled/codeStuff/rust/rustcp/src/${1}/${2}/${3}.rs"
    if [[ ! -f "$target_path" ]]; then
      cat /Users/distiled/codeStuff/rust/rustcp/src/basic.rs >> "$target_path"
      append_to_cargo_toml "$target_path" # Call the function here
    fi
  fi

  if [[ $# == 2 ]]; then
    target_path="/Users/distiled/codeStuff/rust/rustcp/src/${1}/${2}.rs"
    if [[ ! -f "$target_path" ]]; then
      cat /Users/distiled/codeStuff/rust/rustcp/src/basic.rs >> "$target_path"
      append_to_cargo_toml "$target_path" # Call the function here
    fi
  fi
fi
