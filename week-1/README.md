# Week 1 - Integration and Differentiation
> I never get indices right first try.

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

## Exercises

Differentiate and integrate some sine functions.

## Extension

Let's generalize integration to n dimensions so we can use some more complex features of Rust.
