#!/bin/bash

rm -rf target || true
docker build -t libvips-builder .
