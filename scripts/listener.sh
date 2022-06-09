#!/bin/bash

while true; do
    echo "wait for request"
    while test $(curl -s localhost:8881/keys | jq '.list' | wc -l) -eq 1; do
        sleep 1
    done
    echo "key request found, notifying user"
    notify-send "Key request"
    echo "wait until user provide all keys"
    while test $(curl -s localhost:8881/keys | jq '.list' | wc -l) -ne 1; do
        sleep 1
    done
done
