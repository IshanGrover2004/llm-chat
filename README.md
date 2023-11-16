# Large Language Model (LLM) Chatbot

## Table of Contents

- [Overview of LLM](#overview-of-llm)
  - [Introduction](#introduction)
  - [What is LLM?](#what-is-llm-large-language-model)
  - [Key Concepts](#key-concepts)
  - [What LLM can do?](#what-llm-can-do)
  - [Project Overview](#project-overview)
- [Getting Started](#getting-started)
  - [Installation](#installation)
  - [Usage](#usage)
- [Result](#result)
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

### Project Overview

This project utilizes the [OpenLLaMA](https://huggingface.co/rustformers/open-llama-ggml) language model, which is part of the Rustformers model collection.
This LLM is a powerful language model designed for natural language understanding and generation tasks. It can be used to generate text-based responses, provide recommendations, and perform a wide range of language-related tasks.
You can access the model file directly from [Open LLAMA 3B](https://huggingface.co/rustformers/open-llama-ggml/resolve/main/open_llama_3b-f16.bin).

This project offers an API that provides a server to which users can connect their UI components like HTML, CSS, JS. The server processes prompts sent by users, and the Language Model (LLM) generates responses based on these input prompts.

To get started with the project, please refer to the [Usage](#usage) section in this README for detailed instructions.

## Getting Started

### Installation

To set up project locally, run the command to terminal(for Linux) or git bash(for Windows):

```bash
curl -o setup.sh https://raw.githubusercontent.com/IshanGrover2004/llm-chat/master/setup.sh && chmod +x setup.sh && ./setup.sh
```

<table border="1">
    <tr>
        <td><b>Note:</b> The setup process involves downloading a large LLM model binary file of approximately 6.38 GB. The download time may vary depending on your internet connection speed. Be patient while the model file is being downloaded and the repository is set up.</td>
    </tr>
</table>

### Usage

To use this project, first make sure you have [Rust installed](https://www.rust-lang.org/tools/install) on your system.

1. **Start the server:**

   ```bash
   cargo run --release
   ```

   <table border="1">
       <tr>
           <td><b>Note:</b> It is highly recommended to run the project in release mode to achieve optimal performance, resulting in faster and more efficient code execution.</td>
       </tr>
   </table>

2. **Access the server:** Not necessary but recommended  
   Open http://localhost:8080/ in your browser.
   <table border="1">
       <tr>
           <td><b>Note:</b> If you are having problem with availablity of port number then, you can change the server's port number by editing the `config.json` file. The default port is 8080.</td>
       </tr>
   </table>

3. **Interact with the LLM Chatbot:**  
   To interact with the LLM chatbot, you have two convenient options:

   1. **UI Interaction:**
      - Navigate to the `ui` folder and run `index.html`.
      - Use the input box in the UI to write prompts.
      - Wait a little for LLM to generate response.

   2. **Directly using endpoints:**

      - Alternatively, you can interact with the LLM chatbot via server-side endpoints:
        - `http://localhost:8080/chat/my_prompt`
        - `http://localhost:8080/chat?prompt=my_prompt`  
         Simply send your prompt to these endpoints, and the server will provide the LLM-generated response.

## Result
This project is like a toolkit for creating cool things with the Language Model (LLM). The example in the `ui` folder shows how you can use this toolkit to make different stuff in Rust, like chat apps or creative interfaces. I made a basic API using LLM, and the UI demo highlights the variety of projects you can kick off with it. It's an easy way to get started with your creative ideas!
