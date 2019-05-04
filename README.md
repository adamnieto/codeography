# :camera: codeography
---
A CLI that creates beautiful code snippet images using `carbon-now-cli` ([here](https://github.com/mixn/carbon-now-cli)) that can encapsulate as well as generate the source code these code snippets represent pictorially via steganography. 

To understand a little bit better here is a diagram to explain:


![alt text](https://raw.githubusercontent.com/adamnieto/codeography/master/diagram.png)

**Summary:** Generate source code of the code snippet image itself.


## Inspiration and Use Case

* The inspiration for this project comes out of the need for providing customizable syntax highlighting to code snippets through an image without losing the ability to copy and paste the code the code snippet image represents pictorially. Intrinsically, this tool does the copying and pasting for you by simply generating the file from the code image you are looking at.

* This tool can be used to encode a source file of a program but display pictorially the pseudo code of that program.

* This CLI is useful for presentations that may contain code snippets, allowing the presentater avoid styling manually for a presentation but also not sacrafice the ability for viewers to download that code snippet.

---

## Installation:

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

### Encoding a code snippet image to steganographic code snippet

```
codeography <INPUT_CODE_FILE> <INPUT_IMAGE_TO_ENCODE> <ENCODE_OUTPUT_IMAGE_FILE_NAME>
```

Example: 

In this example the CLI will create `codeography_code_snippet.png`.

```
codeography code_snippet.rs code_snippet.png codeography_code_snippet
```

[![asciicast](https://asciinema.org/a/244241.svg)](https://asciinema.org/a/244241)

### Decoding a steganographic code snippet image to its original source code file

```
codeography decode <INPUT_IMAGE_TO_DECODE> <DECODE_OUTPUT_FILE_NAME>
```

Example: 

In this example the program will create `code_snippet.rs`.
```
codeography decode codeography_code_snippet.png code_snippet
```

[![asciicast](https://asciinema.org/a/244243.svg)](https://asciinema.org/a/244243)
