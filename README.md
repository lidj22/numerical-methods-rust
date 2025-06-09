# Numerical Methods in Rust

> from May-June in 2025.

Practice with Rust by implementing some numerical methods in it (preferably from scratch, then maybe with libraries), while also refreshing up on a bit of theory. This will be constrained to a 6-week period.

Some learning outcomes along the way:
- get a better feel of Rust
- become more familiar with computation and visualization libs (equivalents of matplotlib, numpy, scipy, etc.) and do one cool visualization
- read some papers and remember what my degree was

**Bonus points**:
- learn multiprocessing, multi-threading, and/or that "tokio" thing somehow

I consider myself a constructivist learner, and I learn by doing exercises.

## Outline

> This *might* be a bit optimistic...

| Week no. | Week | Topic |
| - | - | - |
| 1 | 5/12 - 5/18 | [Calculus and floating point numbers](#week-1---calculus-and-floating-point-numbers) |
| 2 | 5/19 - 5/25 | [Linear algebra](#week-2---linear-algebra) |
| - | 5/26 - 6/1 | improvised break |
| 3 | 6/2 - 6/8 | Binary arithmetic and CORDIC |
| 4 | 6/9 - 6/15 | Differential equations and simulations in Rust |
| 5 | 6/16 - 6/22 | Navier-Stokes equations (pt. 1) |
| 6 | 6/23 - 6/29 | Navier-Stokes equations (pt. 2) |

I don't actually remember what the NS equations were about. Guess we'll find out on 6/9! Or maybe we can try something else, like molecular dynamics or some interesting differential equation.

I kind of want to put in some ML, but also I don't want to gather data...

## [Week 1 - Calculus and floating point numbers](/week-1/README.md)

Review the *very* basics of numerical differentiation and integration, definition of floating point numbers, IEEE single and double precision definitions. Truncation and rounding errors with numerical differentiation demonstrations in Rust with `f32` and `f64` (i.e. why we need to choose the right step size or implement adaptive step size). 

Overview of differentiation techniques (analytic, numerical, symbolic, automatic). Went through basic Rust data types and logical flows, and basic modules.

### Problems

- Derive the maximum normal, minimum positive, maximum subnormal, and minimum positive subnormal numbers in the IEEE single precision floating point number system.
- Explain why, in the `main.rs` example script, the forward differentiation error for the function $x\mapsto x+1$ approaches $1$ as step size approaches $0$.

## [Week 2 - Linear algebra](/week-2/README.md)

Review of linear algebra topics.

Naive implementation of 3-dimensional matrix struct in Rust, including trace, determinant, transpose, multiplication, addition, Frobenius and max norm. Learn crates, packages, and modules.

## Week 3 - Binary arithmetic and CORDIC (TBD)

Expected outcomes: 
- learn and apply Rust exception handling, generics, traits, and lifecycles.
- CS concepts: binary arithmetic, bitshifts, add, mult
- more application of linear algebra concepts (coordinate transformations, metric tensor)
- start solving some PDEs (heat, wave, laplace, burger, advection).

## Week 4 - Differential equations and simulations (TBD)

## Week 5 - Navier-Stokes 1 (TBD)

## Week 6 - Navier-Stokes 2 (TBD)

## Readings
- An Introduction to Computational Fluid Dynamics: The Finite Volume Method

### Prep
- [How to derive the Navier-Stokes equations](https://cfd.university/learn/10-key-concepts-everyone-must-understand-in-cfd/how-to-derive-the-navier-stokes-equations/)
- [Lecture Notes: Navier-Stokes Equations](https://www.uni-ulm.de/fileadmin/website_uni_ulm/mawi.inst.020/wiedemann/Skripte/EW_Navier-Stokes_Equations.pdf)