#!/usr/bin/env bash

$1
if [ $? -eq 2 ]; then
    echo "Somecommand: success!"
else
    echo "Somecommand: failure!"
fi
