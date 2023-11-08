#!/bin/bash

git clone https://github.com/IshanGrover2004/llm-chat.git && cd llm-chat

download_url="https://huggingface.co/rustformers/open-llama-ggml/resolve/main/open_llama_3b-f16.bin"

download_path="./assets/open_llama_3b-f16.bin"

mkdir -p "./assets/"

wget -O $download_path $download_url

echo "File downloaded successfully and saved to ${download_path}"
