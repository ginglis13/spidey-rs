#!/bin/sh

echo "HTTP/1.0 200 OK"
echo "Content-type: text/plain"
echo

env | sort
