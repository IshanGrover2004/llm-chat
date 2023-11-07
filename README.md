# Large Language Model (LLM) Chatbot

## Table of Contents

- [Overview of LLM](#overview-of-llm)
  - [Introduction](#introduction)
  - [What is LLM?](<#what-is-llm-(large-language-model)?>)
  - [Key Concepts](#key-concepts)
  - [What LLM can do?](#what-llm-can-do)
- [Getting Started](#getting-started)
  - [Installation](#installation)
  - [Usage](#usage)
    <!-- - [Example](#example) -->
    <!-- - [Additional Features](#additional-features) -->
    <!-- - [Contributing](#contributing) -->
    <!-- - [License](#license) -->

## Overview of LLM

### Introduction

This project is a demonstration of a Large Language Model (LLM) chatbot built using Rust and the Axum web framework. It showcases the capabilities of Large Language Models in natural language understanding and generation.

### What is LLM (Large Language Model)?

**Definition:** An LLM, or Large Language Model, is a type of AI model capable of understanding and generating human language text. These models are trained on massive datasets to perform various natural language processing tasks.

**Examples:** BLOOM, GPT-3, LLaMA, PaLM and more.

### Key Concepts

To grasp LLM functionality, it's crucial to understand these key concepts:

- **Pre-training and Fine-Tuning:** LLMs are initially trained on extensive datasets to learn general language understanding. They can be fine-tuned with task-specific data for applications like text classification, translation, and generation.

- **Tokenization:** Tokenization involves breaking text into smaller units, such as words, subwords, or characters, for processing by LLMs.

- **Neural Networks:** These are computational models inspired by the structure and function of the human brain, composed of interconnected artificial neurons organized in layers.

- **Self-Attention Mechanism:** This mechanism allows the model to focus on specific parts of the input sequence or image that are most relevant to the task at hand.

- **Encoding:** Encoding refers to the process of transforming the input text into contextualized word embeddings.

- **Decoding:** Decoding is the process of generating the output sequence, which could be the next word in language modeling or the translated sentence in machine translation tasks.

### What LLM Can Do?

Some applications of LLM includes:

- Natural Language Understanding
- Text Summarization
- Chatbots and Virtual Assistants
- Text to Image Generation
- Question Answering
- Text-to-Speech (TTS)
- Language Modeling

## Getting Started

### Installation

To set up project locally, follow these steps:

1. Clone the repository

```bash
git clone https://github.com/IshanGrover2004/llm-chat.git && cd llm-chat
```

2. Run `setup.sh` script to automate the setup process. This script file will download the LLM model binary file.

```shell
chmod +x setup.sh
./setup.sh
```

### Usage

1. Start the server

```bash
cargo run --release
```

2. Open http://127.0.0.1:8000/ in your browser.

3. Interact with LLM chatbot by entering:

- http://127.0.0.1:8000/chat/my_prompt
- http://127.0.0.1:8000/chat?prompt=my_prompt
