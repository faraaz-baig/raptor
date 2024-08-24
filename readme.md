

# Raptor

**Raptor** is an experimental programming language that compiles to Rust and then to a binary. It aims to bring the ease of Python-like syntax with the performance benefits of Rust. This project is in its early stages and is intended for developers interested in language design, compiler construction, and leveraging Rust for high-performance applications.

## Features

- **Python-like Syntax**: Designed to be familiar and approachable for Python developers.
- **Rust Backend**: Compiles to Rust for high-performance execution.
- **Binary Output**: Generates optimized binary files for deployment.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/learn/get-started) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/) (Rust’s package manager and build system)

### Installation

Clone the repository and build the project:

```bash
git clone https://github.com/faraaz-baig/raptor.git
cd raptor
cargo build
```

### Usage

To compile a RustScript source file, use the following command:

```bash
cargo run -- path/to/your_script.rs
```

This will process the source file and generate the corresponding Rust code. The Rust code will then be compiled into a binary executable.

### Examples

Here is a simple example of a RustScript source file (`examples/hello_world.rs`):

```rap
main() -> None {
    print("Hello, world!")
}
```

Run the following command to compile and execute:

```bash
cargo run -- examples/hello_world.rs
```

### Contributing

Contributions are welcome! If you have suggestions, bug reports, or feature requests, please open an issue or submit a pull request. 

To contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add new feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Open a Pull Request.

### License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Rust](https://www.rust-lang.org/) - For providing a powerful and safe programming language.
- [Cargo](https://doc.rust-lang.org/cargo/) - For managing Rust projects and dependencies.

## Contact

For questions or feedback, please contact fazzy0908@yandex.com.
