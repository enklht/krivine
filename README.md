# krivine

A small Rust implementation of the call-by-need Krivine machine (CS variant), following the optimization from Friedman et al. (2007) that avoids redundant update-marker sequences and yields a more space efficient variant of Krivine’s original K machine.

## Examples

- `cargo run --example addition`
- `cargo run --example factorial`
- `cargo run --example takeuchi`

## References

- Jean-Louis Krivine, “A call-by-name lambda-calculus machine,” *Higher-Order and Symbolic Computation* 20(3), 2007, pp. 199–207. DOI: <https://doi.org/10.1007/s10990-007-9018-9>
- Daniel P. Friedman, Abdulaziz Ghuloum, Jeremy G. Siek, Onnie Lynn Winebarger, “Improving the lazy Krivine machine,” *Higher-Order and Symbolic Computation* 20(3), 2007, pp. 271–293. DOI: <https://doi.org/10.1007/s10990-007-9014-0>
