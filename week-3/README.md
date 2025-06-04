# Week 3

## Rust - Matrices (Take 2)

We start by re-implementing the matrix library from Week 2, with a greater emphasis on traits, generics, and exception handling.

To motivate everything here, we begin by defining a single trait for matrix multiplication. Then, realize that during testing, we need to assert that two matrices are equal. How to do this?

We can do element-wise array comparison. However, after iterations, this may be inaccurate. One way to do this would be to use:

$$
A \approx B \iff \|A-B\| \approx 0.
$$

This leads us to implementing norm and subtraction, which leads us to implementing addition and negative.

> Notes: I've attempted to try `ops::Add` and `ops::Neg` for the sake of idiomatic Rust code, but they seem to go against everything I've written so far (in terms of requiring mixing references and taking ownership) so I've decided not to implement them here.

As we implement the required operations, we can slowly do the rest (e.g. transpose, scalar multiplication).

Then, while implementing the norm, we need to consider the trivial case: $0\times 0$ matrices (or empty matrices). This needs to throw error. Thus, we go into error handling, case matching and the Result/Err pattern.

Also for `approx_equals`, how do we set a default parameter to, say 0.0001? Same goes for norms.

At this point, all the code was in one file, so we might look at how to split the code.

The aforementioned operations are all for generic matrices of arbitrary dimension. But what about square matrices?
