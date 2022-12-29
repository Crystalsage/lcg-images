# Linear Congruential Generators
A pseudo-random number generator that uses a piecewise linear equation defined as follows:

$$
X_{n+1} = (aX_n + c) \pmod{m}
$$

Where:
- X is a sequence of pseudo-random values,
- $m$ is the modulus.
- $a$ is the multiplier.
- $c$ is the increment.
- $X_0$ is a seed.


## The experiment
The image generated with this program uses the [Netbpm image format's](https://en.wikipedia.org/wiki/Netpbm) P6 variation. 

Every pixel in the image has a *twin*. If one pixel is at $Y_{n+1} = X_n * m + c \pmod{m}$ 
then another would be at $Y'_{n+1} = X_n' * m + c$ where $X' = X^{-1} \pmod{m}$.

## Results
TODO
