#!/bin/bash

cargo snippet -t vscode | sed  -r  " s/ \" prefix \" / \" scope \" : \" rust \" , \n \" prefix \" / " > .vscode/cargo_snippet.code-snippets
