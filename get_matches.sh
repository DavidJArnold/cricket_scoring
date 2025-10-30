#!/bin/bash

# curl -o assets/matches.zip "https://cricsheet.org/downloads/recently_added_30_json.zip"
curl -o matches.zip "https://cricsheet.org/downloads/all_json.zip"
unzip matches.zip -d examples/all_matches
