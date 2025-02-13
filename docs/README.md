# Westem
This is a simple calculator program written in Rust that performs basic arithmetic operations. It takes three command-line arguments: the left operand, the operator, and the right operand. The program then calculates the result of the operation and prints it to the console.

## Installation


To build and run the program, you need to have Rust installed on your system. You can download and install Rust from the official website: https://www.rust-lang.org/tools/install

Once you have Rust installed, you can clone the repository and build the program using Cargo:

```bash
git clone https://github.com/RickFerrDev/westem

cd westem
cargo build --release
```

## Usage
To run the program, use the following command:
```bash
./target/release/westem <left_operand> <operator> <right_operand>
```

For example, to calculate the sum of 5 and 3, you would use the following command:
```bash
./target/release/westem 5 + 3
```

The program supports the following operators:
```
 + for addition
 - for subtraction
 / for division
 * for multiplication
```

## Documentation
The code is documented using Rust's documentation syntax. You can generate the documentation using the following command:
```bash
cargo doc
```

The documentation will be generated in the target/doc directory. You can open the index.html file in your browser to view the documentation.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request if you have any suggestions or bug fixes.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.
