### Rust File Compression Library

A blazing fast file compression tool written in Rust.

#### Usage
This assumes you have Rust installed on your machine. If you don't, you can get it [here](https://www.rust-lang.org/tools/install).
Vs code too if you want to use the code editor.

```bash
git clone https://github.com/frankudoags/project-compression.git
cd rust-file-compression
code .
```

Add the file you want to compress to the root directory.

Then run the following command in the terminal:

```bash
cargo run {filename to compress} {filename to save to}
```

#### Example

```bash
touch test.txt
cargo run test.txt test.txt.compressed
```

Compresses the file test.txt and saves it as test.txt.compressed

Also shows the compression ratio, and the time it took to compress the file.
