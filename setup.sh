#!/bin/bash

download_url="https://huggingface.co/rustformers/open-llama-ggml/resolve/main/open_llama_3b-f16.bin"

download_path="./assets/open_llama_3b-f16.bin"

mkdir -p "./assets/"

wget -P $download_path $download_url

if mycmd; then
	echo "File downloaded successfully and saved to ${download_path}"
else
	echo "Failed to download the file."
fi
