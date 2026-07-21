#!/bin/bash

#git add .
#git commit -m "$1"
git -c http.proxy=http://192.168.113.80:8118  push origin HEAD -uf
