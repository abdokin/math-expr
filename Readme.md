# Math Expression Evaluator in Rust

This is a basic Math Expression Evaluator implemented in Rust.

## Features

- Evaluates simple mathematical expressions.
- Supports addition (+), subtraction (-), multiplication (*), division (/), and parentheses for precedence.
- Provides a command-line interface (CLI) for interactive evaluation.

## Usage

### Installation

To build and run the Math Expression Evaluator, ensure you have Rust installed. If not, follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).

### Running the Evaluator

Clone this repository:

```bash
git clone https://github.com/abdokin/math-expr.git
```

Navigate to the project directory:

```bash
cd math-expr
```

Build and run the evaluator:

```bash
cargo run
```

### Example Usage

Once the evaluator is running, it will prompt you to enter a mathematical expression:

```
Enter a mathematical expression to evaluate or type ':quit' to exit:
>> 5 + 3 * (6 - 2)
20
>> 10 / (2 + 3) * 4
8
>> :quit
```

The evaluator will compute the result of the entered expression. Type `:quit` to exit the evaluator.

## Contributions

Contributions are welcome! Feel free to open issues for suggestions or bug reports. Pull requests are also appreciated.

## License

This project is licensed under the [MIT License](LICENSE).

---

Feel free to expand on this README with more details about the implementation, additional features, supported expressions, performance optimizations, or any other relevant information about your Math Expression Evaluator in Rust.