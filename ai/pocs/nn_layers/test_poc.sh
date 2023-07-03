#!/bin/bash

# This script is used to test the model launcher and data viewer
../../ai_model_building/generate_models.sh
python3 data_viewer.py
python3 model_launcher.py $1 "match_sample.json"
