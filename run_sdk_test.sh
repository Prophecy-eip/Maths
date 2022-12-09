#!/bin/sh

clear

cargo fmt

cargo build

tsc sdk/typescript/src/index.ts

node sdk/typescript/src/index.js
