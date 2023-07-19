#!/bin/bash

# This script is used to test the model launcher and data viewer
python3 data_viewer.py
python3 model_launcher.py $1 "match_sample.json"
