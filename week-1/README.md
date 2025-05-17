# Week 1 - Integration and Differentiation
> I never get indices right first try.
> I also remember now, my notes are a mess. Just read the references

Let $f$ be nice.

## Theory

### Differentiation
The derivative of $f$ at point $x$ can be approximated with timestep $h > 0$:

$$
\dfrac{df}{dx}\approx\dfrac{\Delta f}{\Delta x} = \dfrac{f(x+h) - f(x)}{h}
$$

So this is a pretty straightforward implementation.

Other one is the average:

$$
\dfrac{df}{dx}\approx\dfrac{f(x+h) - f(x-h)}{2h}
$$

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

### Numerical Analysis

Definition of floating point number:

$$
\bar{x} = (-1)^s\bigg(\sum_{i=0}^{m-1}d_i\beta^{-1}\bigg)\beta^e
$$

basically scientific notation.

Think about what it means for a number to be "normalized."

exercise: does floor: f32 -> i32 exist? No (check range).

IEEE Definitions:
- single precision floating point number (32 bits)
- double precision floating point number (64 bits)

32 bit standard: 1 bit for sign, 8 bits exponent, 23 bits mantissa.

64 bit standard: 1 bit sign, 11 bits exponent, 52 bits mantissa

exercise: why do we have subnormal floating point numbers?

exercise: derive the "important numbers" for the IEEE floating point numbers section. In other words: how did they come up with these numbers

## Exercises

Differentiate and integrate some sine functions.

## Extension

Let's generalize integration to n dimensions so we can use some more complex features of Rust.

## Readings

- [Floating Point Arithmetic](https://www2.math.uconn.edu/~leykekhman/courses/MATH_3511/sp_2012/Lectures/floating_point.pdf)
- [What Every Computer Scientist Should Know about Floating-Point Arithmetic](https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html)
- [Numerical Differentiation lecture notes](https://math.umd.edu/~dlevy/classes/amsc466/lecture-notes/differentiation-chap.pdf)
