# GeneZip 🧬

GeneZip is a Rust command-line tool designed to compress files containing strings of genetic information using Huffman encoding. Huffman encoding is an efficient method for compressing data by assigning variable-length codes to input characters based on their frequencies.

## Features

- Compress genetic information files using Huffman encoding
- Decompress files compressed with GeneZip
- Simple and intuitive command-line interface

<!--Not yet implemented-->

<!--## Installation

To install GeneZip, you need to have Rust and Cargo installed on your machine. If you don't have them installed, you can get them from [rust-lang.org](https://www.rust-lang.org/).

Once Rust and Cargo are installed, you can install GeneZip using the following command:

```
cargo install genezip
```

## Usage

GeneZip provides two main commands: `compress` and `decompress`.

### Compress

To compress a file, use the `compress` command followed by the input file and the desired output file:

```
genezip compress <input_file> <output_file>
```

**Example:**

```
genezip compress sequences.txt sequences.gzp
```

### Decompress

To decompress a file, use the `decompress` command followed by the compressed file and the desired output file:

```
genezip decompress <input_file> <output_file>
```

**Example:**

```
genezip decompress sequences.gzp sequences.txt
```

## Example

### Compressing a File

Suppose you have a file `genetic_data.txt` containing strings of genetic information. To compress this file, run:

```
genezip compress genetic_data.txt genetic_data.gzp
```

### Decompressing a File

To decompress the previously compressed file `genetic_data.gzp`, run:

```
genezip decompress genetic_data.gzp genetic_data_decompressed.txt
```

## Contributing

Contributions are welcome! If you have suggestions for improvements or new features, feel free to open an issue or submit a pull request. Please ensure your code adheres to the existing code style and includes appropriate tests.

## License

GeneZip is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information. --> 
