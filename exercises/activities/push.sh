#!/bin/bash

git add .
git commit -m "$1"
git -c http.proxy= -c https.proxy=  push origin HEAD -u
