# Math Expression Evaluator in Rust

This is a versatile Math Expression Evaluator implemented in Rust that offers robust functionality for handling mathematical expressions.

## Features

- **Basic Operations:** Evaluates simple mathematical expressions with support for addition (+), subtraction (-), multiplication (*), and division (/).
- **Operator Precedence:** Utilizes parentheses to define the precedence of operations, ensuring accurate evaluation.
- **Trigonometric Functions:** Includes trigonometric functions like cosine (cos), sine (sin), tangent (tan), arccosine (acos), arcsine (asin), and arctangent (atan).
- **Interactive CLI:** Provides an interactive command-line interface for convenient expression evaluation.

## Usage

### Installation

1. **Clone the Repository:**

    ```bash
    git clone https://github.com/abdokin/math-expr.git
    ```

2. **Navigate to the Project Directory:**

    ```bash
    cd math-expr
    ```

3. **Build and Run the Evaluator:**

    ```bash
    cargo run
    ```

### Example Usage

1. **Running the Evaluator:**

    Once the evaluator is running, it prompts users to enter a mathematical expression:

    ```
    Enter a mathematical expression to evaluate or type ':quit' to exit:
    >> 5 + 3 * (6 - 2)
    Result: 17
    >> 10 / (2 + 3) * 4
    Result: 8
    >> cos(0) + sin(0)
    Result: 1
    >> :quit
    ```

2. **Exiting the Evaluator:**

    To exit the evaluator, type `:quit`.

## Contributions

Contributions, suggestions, bug reports, and enhancements are welcomed! Open issues to discuss ideas or report issues. Pull requests are appreciated.

## License

This project is licensed under the [MIT License](LICENSE).

---

Feel free to further enrich this README with detailed explanations, usage examples, supported functions, or any other pertinent information about your Math Expression Evaluator in Rust.