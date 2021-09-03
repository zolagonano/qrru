# qrru

![b](https://img.shields.io/crates/l/qrru)
![b](https://img.shields.io/crates/d/qrru)
![b](https://img.shields.io/crates/v/qrru)
![b](https://img.shields.io/docsrs/qrru)

qrru is a CLI tool to encode and decode qr-codes

## Installation:

### From cargo:

```bash
cargo install qrru
```

### Manually:

1. Clone the repository:

```bash
git clone https://github.com/zolagonano/qrru.git
```

2. Change your working directory to source code directory:

```bash
cd qrru
```

3. Compile and run:

```bash
cargo build --release
cargo run
```

## Usage:

### Encoding:

To encode your text into QRcode, you  need to run the command below:

```bash
qrru encode <Your Text> -o <Output File>
```

Also, you can change the width and height of the output image with `--width`/`-w` and `--height`/`-h` arguments.

### Decoding:

The decoding process is pretty much like the encoding, You have to give your QRcode image to the program.

```bash
qrru decode <Qr Image>
```

## Contribute:
All contributions are welcome but if you don't know what you can do look at this list:

- Open an issue if you find a bug.
- Open an issue if you have a suggestion.
- Fix bugs and send pull requests.
- Share it with your friends.
- And anything else you think will help this project :)
