# :camera: codeography
---
A CLI that creates code snippet images using `carbon-now-cli` ([here](https://github.com/mixn/carbon-now-cli)), however, it can also encapsulate and generate the source code these code snippets represent pictorially via steganography. 

To understand a little bit better here is a diagram to explain:


![alt text](https://raw.githubusercontent.com/adamnieto/codeography/master/diagram.png)

## What is Steganography? 

Steganography is the process of representing information within another message or physical/virtual object in such a way that is not evident in plain sight or after human inspection.

In other words, steganography is a way for us to hide messages within everyday files or objects. 

## Inspiration and Use Case

* The inspiration for this project comes out of the need for providing customizable syntax highlighting to code snippets through an image without losing the ability to copy and paste the source code the code-snippet image represents pictorially. Intrinsically, this tool does the copying and pasting for you by simply generating the file from the code-snippet image you already have.

* For instance, this tool can be used to encode a source file of a program but display pictorially the pseudocode of that program.

* This CLI is useful for presentations that may contain code snippets, allowing the presenter to avoid styling the code manually for a slide but also not burden viewers in having to download a separate file, or copy/paste (formatting may screw up) the code snippet.

---

## Installation:

### Installing using Dev Container (recommended)

**Pre-requisites:** Please make sure that you have Docker installed on your machine and you are using Visual Studio Code. 

1. Download this repo:

```bash
git clone https://github.com/adamnieto/codeography.git
```
2. Wait for the dev container to finish running including the `postCreateCommand` once that has finished it should have installed the `codeography` command and you can start using it.

### Installing Manually

1. Make sure that you have already installed the `carbon-now-cli`. Follow the installation rules [here](https://github.com/mixn/carbon-now-cli).

2. [Install Rust](https://www.rust-lang.org/tools/install/)

3. Download this repo:

```bash
git clone https://github.com/adamnieto/codeography.git

```

4. Once inside of the `codeography` directory run the following command: 

```
cargo install --path .
```

---
## Usage:

### Creating a code snippet image

To create a code snippet image all you need to execute is the following command:

```
codeography imagify <INPUT_CODE_FILE>
```

Example: 

In this example, codeography will delegate to the `carbon-now-cli` in interactive mode.

```
codeography imagify code_snippet.rs
```

[![asciicast](https://asciinema.org/a/244240.svg)](https://asciinema.org/a/244240)

### Encoding a code-snippet image to steganographic code-snippet image

```
codeography encode <INPUT_CODE_FILE> <INPUT_IMAGE_TO_ENCODE> <ENCODE_OUTPUT_IMAGE_FILE_NAME>
```

Example: 

In this example the CLI will create `codeography_code_snippet.png`.

```
codeography encode code_snippet.rs code_snippet.png codeography_code_snippet
```

[![asciicast](https://asciinema.org/a/244241.svg)](https://asciinema.org/a/244241)

### Decoding a steganographic code-snippet image to its original source code file

```
codeography decode <INPUT_IMAGE_TO_DECODE> <DECODE_OUTPUT_FILE_NAME>
```

Example: 

In this example the program will create `code_snippet.rs`.
```
codeography decode codeography_code_snippet.png code_snippet
```

[![asciicast](https://asciinema.org/a/244243.svg)](https://asciinema.org/a/244243)

---
## Notes

This project is still in its early stages. If you find any bugs please report them in the issues tab. 


