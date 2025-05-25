# Week 2 - Linear Algebra (TBD)
These titles are just guidelines.

## Theory

### Linear Algebra Review

Review: these words should mean something to you (albeit a bit hazy):
- vector space
- triangle inequality
- invertible matrix theorem
- symmetric/anti-symmetric matrix
- linear independence
- range, null space
- projection
- trace, determinant, rank
- quadratic forms, PD, PSD, ND, NSD, Gram matrix
- eigenvalues and diagonalization
- gradient and Hessian (for multi-variable, single-valued functions)

Exercise: let $A = \begin{bmatrix}1 & 2\\3 & 2\end{bmatrix}$. find the characteristic polynomial, eigenvalues, eigenvectors, and invertible $P$ such that $P^{-1}AP$ is diagonal.

Exercise: Implement an f64-valued Matrix data structure in 3 dimensions: `SquareMatrix3` (and see if can extend to arbitrary dimensions). Define the basic arithmetic operations for matrices. Implement trace and determinant. Put this in the `linear_algebra` mod and make it available as part of the `week-2` library crate. Add some tests.

### Tensor Calculus (TODO)

TODO: coordinate transformations, metric tensor

Exercise: do some recommended exercises from Ch1.

### CS (Non-rust related)
Exercise: Run `xxd README.md` and and interpret the lines.

Exercise: some model tags in Ollama have the label "Q4". Why is it not called "FP4" like in "FP16"?

### Rust Language

Go over rust crates and packages, crate roots, binary vs. library crates, modules and paths. The physical/logical distinction between crates and modules. 

Exercise: can a *strict* subblock of code within a file be a crate? Can a module span multiple files?

Exercise: we establish that crate roots are a property of a crate. Given a crate, does a crate root necessarily exist and is unique? Does a package with n binary crates and 0/1 library crates have n+0/1 crate roots?

Exercise: for the following structures, do they comprise a conventionally valid package? If not, why?
```
# problem 1
my_package/
├── Cargo.toml
├── src/
    └── bin/
        └── package_bin.rs     # Binary crate
```
```
# problem 2
my_package/
├── Cargo.toml
├── src/
    └── hello/
        └── world/
            └── lib.rs
```
```
# problem 3
my_package/
├── Cargo.toml
├── src/
    └── bin/
        └── package_bin.rs      # Binary crate
    └── lib_1/
        └── lib.rs
    └── lib_2/
        └── lib.rs
```
```
# problem 4
my_package/
├── Cargo.toml
├── src/
    └── main.rs
    └── lib.rs
```

Exercise: what is the smallest ".rs" file count for a package that contains both a binary and a library crate?

Exercise: identify the crate roots, binary crates, and the library crate of the [rust-lang/cargo](https://github.com/rust-lang/cargo) git repository. Draw the folder structure for two more binary crates.

Review: what is a path and what is a module? In the crate root, when declaring modules, where (positionally) does compiler search for a module?

Exercise: can you declare modules elsewhere than the crate root file, which are not submodules?

### Performance

The naive implementation of rust takes 2.8 SECONDS to run the dynamical system, which is SLOWER than the vanilla python implementation (2.3s). However an optimized version takes as little as 0.004 seconds!!

Some things to look at:
- heap allocations
- derive: copy, clone
- inline
- function call overheads
- using `--release` flag

## Readings
- [Examples and Sample Problems](https://courses.ics.hawaii.edu/ReviewICS312/morea/X86NASM/ics312_samples.pdf)
- [Endianness Lecture Notes](https://cs3157.github.io/www/2023-9/lecture-notes/14-endian.pdf)
- [Invertible Matrix Theorem](https://mathworld.wolfram.com/InvertibleMatrixTheorem.html)
- [Review of Linear Algebra](https://cs229.stanford.edu/section/cs229-linalg.pdf)
- [A Brief Introduction to Numerical Methods for Differential Equations](https://www.lehman.edu/academics/cmacs/documents/NumericalIntegrationTutorial.pdf)
- [Introduction to Tensor Calculus](https://www.ita.uni-heidelberg.de/~dullemond/lectures/tensor/tensor.pdf)

Unrelated, but is an interesting read.
- [Inverse Problems](https://tristanvanleeuwen.github.io/IP_and_Im_Lectures/intro.html)
