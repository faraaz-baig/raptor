

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

To compile a Raptor source file, use the following command:

```bash
cargo run -- path/to/your_script.rs
```

This will process the source file and generate the corresponding Rust code. The Rust code will then be compiled into a binary executable.

### Examples

Here is a simple overview of a raptor lang (`examples/hello_world.rs`):

```rap
# Variable declarations
let x = 5
let y = 10.5
let name = "Alice"

# Function definition
fn add(a: int, b: int) -> int:
    return a + b

# Mutable variables
let mut counter = 0

# Loops
while counter < 5:
    print(counter)
    counter = counter + 1

# Conditionals
if x > 3:
    print("x is greater than 3")
else:
    print("x is not greater than 3")

# Lists (will be translated to Rust Vec)
let numbers = [1, 2, 3, 4, 5]

# For loop (iteration over lists)
for num in numbers:
    print(num)

# Structs
struct Person:
    name: string
    age: int

# Creating an instance of a struct
let alice = Person{name: "Alice", age: 30}

# Match expression (similar to Rust's match)
match x:
    1 -> print("One")
    2 -> print("Two")
    _ -> print("Other")

# Error handling (using Result type)
fn divide(a: float, b: float) -> Result<float, string>:
    if b == 0:
        return Err("Division by zero")
    return Ok(a / b)

# Using the Result
let result = divide(10, 2)
match result:
    Ok(value) -> print("Result: " + str(value))
    Err(msg) -> print("Error: " + msg)

# Importing modules
import math

# Using imported functions
let sqrt_result = math.sqrt(16)
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
