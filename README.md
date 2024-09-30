## Overview
- This project implements basic operations on polynomials with integer coefficients modulo `n` using the Rust programming language. The operations include:
   - Addition of polynomials.
   - Multiplication of polynomials.
   - Degree calculation of a polynomial.
- These operations are performed on polynomials where the coefficients are integers modulo a given integer `n`.
## Features
- **Polynomial Addition:** Adds two polynomials coefficient-wise and reduces the result modulo `n`.
- **Polynomial Multiplication:** Multiplies two polynomials using the distributive property, and reduces the coefficients modulo `n`.
- **Degree Calculation:** Determines the degree of the polynomial, which is the highest power of the variable with a non-zero coefficient.

## Requirements
- Rust installed on your machine. (If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it).
## Example Polynomials
- Input two polynomials for addition or multiplication:
>```
>Enter the first polynomial (e.g., 5x^2-4x + 2): 5x^2-4x + 2
>Enter the second polynomial (e.g., x^3-2x^2 + 5): x^3-2x^2 + 5

## Example Output:
>```
>P(x) + Q(x) mod 6 = [1, 2, 3, 1]
>P(x) * Q(x) mod 6 = [4, 4, 3, 4, 4, 5]
>deg(P + Q) = 3
>deg(P * Q) = 5

## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

## Usage
- To use this code, you can clone the repository.
- Compile the Rust code using cargo:
>```
>cargo build
>cargo run

## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
   git clone https://github.com/Polynomial_Addition_Multiplication_and_Degree_Calculation_in_Z_n_with_Rust.git
   cd Polynomial_Addition_Multiplication_and_Degree_Calculation_in_Z_n_with_Rust
