#! /usr/bin/env bash

# Fetch the latest version of the avilla repo
git submodule update --init --recursive
git submodule foreach 'git pull'

# Build avilla core
cd avilla
python -m pdm mina build core -vv
cd ..

# Install avilla core
pip install avilla/dist/avilla_core-1.0.0rc3-py3-none-any.whl