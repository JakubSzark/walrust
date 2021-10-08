@ECHO off

cd runtime
start wasm-pack build -d ../editor/pkg --target web