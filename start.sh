#!/bin/sh

wasm-pack build
cd www
[ ! -d "node_modules" ] && npm install
npm run start &
