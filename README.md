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
| 2 | 5/19 - 5/25 | Linear algebra |
| 3 | 5/26 - 6/1 | Binary arithmetic and CORDIC |
| 4 | 6/2 - 6/8 | Gierer Meinhardt system. This is where you might want to do some visualizing. |
| 5 | 6/9 - 6/15 | Navier-Stokes equations (pt. 1) |
| 6 | 6/16 - 6/22 | Navier-Stokes equations (pt. 2) |

I don't actually remember what the NS equations were about. Guess we'll find out on 6/9! Or maybe we can try something else, like molecular dynamics or some interesting differential equation.

I kind of want to put in some ML, but also I don't want to gather data...

## [Week 1 - Calculus and floating point numbers](/week-1/README.md)

Review the *very* basics of numerical differentiation and integration, definition of floating point numbers, IEEE single and double precision definitions. Truncation and rounding errors with numerical differentiation demonstrations in Rust with `f32` and `f64` (i.e. why we need to choose the right step size or implement adaptive step size). 

Overview of differentiation techniques (analytic, numerical, symbolic, automatic). Went through basic Rust data types and logical flows, and basic modules.

### Problems

- Derive the maximum normal, minimum positive, maximum subnormal, and minimum positive subnormal numbers in the IEEE single precision floating point number system.
- Explain why, in the `main.rs` example script, the forward differentiation error for the function $x\mapsto x+1$ approaches $1$ as step size approaches $0$.

## [Week 2 - Linear algebra (TBD)](/week-2/README.md)

Expected (TBD): follow up on learning floating point arithmetic: rounding theorem (numerical analysis), big/little endianness. And read Goldberg.

Expected 2: start with review of linear algebra -> numerical linear algebra. Then a review of differential equations (eigenvalues/eigenvectors) -> dynamical systems -> numerical methods of solving DEs and PDEs. Then, follow up with a survey of modern dynamical systems approaches. Only meant as a survey of techniques, not comprehensive (and might drag on to week 3).

> Also put some focus in vector and tensor algebra: this will probably be important later on.

> If we're doing the navier stokes, we might want to start with the derivation sooner.

Outcome (TBD): design and implement a matrix or tensor struct in Rust. Implement numerical methods like determinant, matrix multiplication and addition. Solve systems of differential equations using Euler and RK4.

## Week 3 - TBD

## Readings
- An Introduction to Computational Fluid Dynamics: The Finite Volume Method

### Prep
- [How to derive the Navier-Stokes equations](https://cfd.university/learn/10-key-concepts-everyone-must-understand-in-cfd/how-to-derive-the-navier-stokes-equations/)
- [Lecture Notes: Navier-Stokes Equations](https://www.uni-ulm.de/fileadmin/website_uni_ulm/mawi.inst.020/wiedemann/Skripte/EW_Navier-Stokes_Equations.pdf)