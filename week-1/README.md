# Week 1 - Integration and Differentiation

## Theory

### Numerical Analysis - Floating Point Numbers

Definition of floating point number:

$$
\bar{x} = (-1)^s\bigg(\sum_{i=0}^{m-1}d_i\beta^{-1}\bigg)\beta^e
$$

basically scientific notation.

Think about what it means for a number to be "normalized."

Exercise: does floor: f32 -> i32 exist? No (check range).

IEEE Definitions:
- single precision floating point number (32 bits)
- double precision floating point number (64 bits)

32 bit standard: 1 bit for sign, 8 bits exponent, 23 bits mantissa.

64 bit standard: 1 bit sign, 11 bits exponent, 52 bits mantissa

Exercise: why do we have subnormal floating point numbers?

Exercise: derive the "important numbers" for the IEEE floating point numbers section. In other words: how did they come up with these numbers

### Numerical Differentiation
The derivative of $f$ at point $x$ can be approximated with timestep $h > 0$:

$$
\dfrac{df}{dx}\approx\dfrac{\Delta f}{\Delta x} = \dfrac{f(x+h) - f(x)}{h}
$$

This is "forward differencing". We also have "backward differencing":

$$
\dfrac{df}{dx}\approx\dfrac{f(x) - f(x-h)}{h}
$$

So this is a pretty straightforward implementation.

Other one is "centered differencing" which is usually better:

$$
\dfrac{df}{dx}\approx\dfrac{f(x+h) - f(x-h)}{2h}
$$

These are also referred to as "finite difference" methods. Naturally, there are issues with accuracy/stability (which will become apparent when implementing numerical differentiators in Rust).

Discussion: truncation error vs rounding error, step size dilemma.

**Differentiation by interpolation**: Review the Interpolation theorem and the Lagrange interpolating polynomial. Given a set of data points, we differentiate the interpolant.

There are other differentiation methods (method of undetermined coefficients, Richardson's extrapolation).

### Symbolic Differentiation (TBD)

Review: conditions for a function to be symbolically differentiable.

TODO: expression swell of symbolic expressions

### Automatic Differentiation

Review: Jacobian matrix

An implementation of differentiation (autodiff) that also operates on program of interest: it operates on the intermediate implementation of a function.

Video: [What is Automatic Differentiation?](https://www.youtube.com/watch?v=wG_nF1awSSY)

Exercise: This is a good one from the video. Given

$$
f(x_1, x_2) = \bigg[\sin\bigg(\dfrac{x_1}{x_2}\bigg) + \dfrac{x_1}{x_2} - e^{x_2}\bigg]\times\bigg[\dfrac{x_1}{x_2} - e^{x_2}\bigg]
$$

find $\dfrac{\partial f}{\partial x_1}\bigg\vert_{(1.5, 0.5)}$ using automatic differentiation (by HAND).

Discussion: forward vs reverse mode of autodiff (when is which more ideal).

Link to the autograd paper/thesis: [Modeling, Inference and Optimization with Composable Differentiable Procedures](https://dougalmaclaurin.com/phd-thesis.pdf). See Chapter 4.

### Integration

The integral of $f$ from $a$ to $b$ can be approximated by a left Riemann sum with constant timestep $h > 0$. 

The number of sub-intervals is then

$$
n = \dfrac{b - a}{h}
$$

So we can have the following formula for a left Riemann sum:

$$
\sum_{i=0}^n f(a + ih)\cdot h
$$

> Also probably $n$ should be a floor or ceiling, we should probably figure that part out.

## Exercises

Differentiate and integrate some sine functions.

## Extension

Let's generalize integration to n dimensions so we can use some more complex features of Rust.

## Readings

- [Floating Point Arithmetic](https://www2.math.uconn.edu/~leykekhman/courses/MATH_3511/sp_2012/Lectures/floating_point.pdf)
- [What Every Computer Scientist Should Know about Floating-Point Arithmetic](https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html)
- [Numerical Differentiation lecture notes](https://math.umd.edu/~dlevy/classes/amsc466/lecture-notes/differentiation-chap.pdf)
