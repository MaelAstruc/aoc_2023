This file is used to keep track of the different estimations and formulas. I found the first values using rust and estimated the results in an excel file.

I will maybe do all the cycle detection and estimations in Rust at some point but not today.

## General idea

The number of garden plots we can reach after each step increases exponentially.

After computing the difference, we can observe a linear trend with noise around it.

Once we remove the trend, we see a cyclicity in the noise: the spikes seem to appear after 131 steps.

However, the spikes seem to be larger after each cycle.

Hence, we compute the difference in the spikes size after each cycle and see once again a linear trend.

Once we know this, we can determine the number of garden plot we can reach from the base values and the trends.

Let's define $x_n$ the number of garden plots we can reach after $n$ steps.

We compute $d_n = x_n - x_{n-1}$ for $n > 0$.

We then compute $\Delta_n = d_n - d_{n-1}$.

For a step $n$, we define $m(n) = n % C$ the number of cycles passed and $c(n) = n // C$ the advancement in the last cycle of size $C$. We define $c_i$ the number cycle step.

We can redefine $\Delta_n = \Delta_{m(n), c(n)} = \alpha_{c(n)} + m(n) \beta_{c(n)}$

With:

- $\alpha_{c(n)}$: the value of $\Delta_n$ for cycle step $c(n)$ when $n < C$
- $\beta_{c(n)}$: the trend for cycle step $c(n)$

## Definitions

We define

- $A(k) = \sum_{i=0}^{k} \alpha_{c_i}$
- $B(k) = \sum_{i=0}^{k} \beta_{c_i}$
- $A = \sum_{i=0}^{C-1} \alpha_{c_i}$
- $B = \sum_{i=0}^{C-1} \beta_{c_i}$
- $X(k) = \sum_{i=0}^{k} i \alpha_{c_i}$
- $Y(k) = \sum_{i=0}^{k} i \beta_{c_i}$
- $X = \sum_{i=0}^{C-1} i \alpha_{c_i}$
- $Y = \sum_{i=0}^{C-1} i \beta_{c_i}$
- $U(k) = \sum_{i=0}^{k} i^2 \alpha_{c_i}$
- $V(k) = \sum_{i=0}^{k} i^2 \beta_{c_i}$
- $U = \sum_{i=0}^{C-1} i^2 \alpha_{c_i}$
- $V = \sum_{i=0}^{C-1} i^2 \beta_{c_i}$

## Intermediary formulas

Some intermediary formulas I tried along the way

- $\sum_{i=0}^n m(i)$

	$= \sum_{i=0}^{m(n)-1} \sum_{j=0}^{C-1} i + \sum_{j=0}^{c(n)} m(n)$

	$= \sum_{i=0}^{m(n)-1} C i + c(n) m(n)$

	$= C \sum_{i=0}^{m(n)-1} i + c(n) m(n)$

	$= C m(n) (m(n) - 1) / 2 + c(n) m(n)$

- $\sum_{i=0}^n m(i)^2$

	$= \sum_{i=0}^{m(n)-1} \sum_{j=0}^{C-1} i^2 + \sum_{j=0}^{c(n)} m(n)^2$

	$= \sum_{i=0}^{m(n)-1} C i^2 + c(n) m(n)^2$

	$= C \sum_{i=0}^{m(n)-1} i^2 + c(n) m(n)^2$

	$= C m(n) (m(n) - 1) (2m(n) - 1) / 6 + c(n) m(n)^2$

- $\sum_{i=0}^n m(i)^2 - m(i)$

	$\sum_{i=0}^n m(i)^2 - \sum_{i=1}^n m(i)$

	$= [ C m(n) (m(n) - 1) (2m(n) - 1) / 6 + c(n) m(n)^2 ] - [ C m(n) (m(n) - 1) / 2 + c(n) m(n) ]$

	$= C [ m(n) (m(n) - 1) (2m(n) - 1) / 6 - m(n) (m(n) - 1) / 2 ] + c(n) [ m(n)^2 - m(n) ]$

	$= C m(n) (m(n) - 1) [ (2m(n) - 4 ] / 6 + c(n) m(n) (m(n) - 1) $
	
	$= C m(n) (m(n) - 1) (m(n) - 2) / 3 + c(n) m(n) (m(n) - 1)$

	$= m(n) (m(n) - 1) [ C (m(n) - 2) / 3 + c(n) ]$

- $\sum_{i=0}^n i m(i)$

	$= \sum_{i=0}^{m(n)-1} \sum_{j=0}^{C-1} (i * C + j) i + \sum_{j=0}^{c(n)} (m(n) C + j) m(n)$

	$= \sum_{i=0}^{m(n)-1} i [i C^2 + \sum_{j=0}^{C-1} j] + m(n) [ m(n) C (c(n) + 1) + \sum_{j=0}^{c(n)} j ]$

	$= \sum_{i=0}^{m(n)-1} i [i C^2 + C (C - 1) / 2 ] + m(n) [ m(n) C (c(n) + 1) + c(n) (c(n) + 1) / 2 ]$

	$= C^2 \sum_{i=0}^{m(n)-1} i^2 + C (C - 1) / 2 * \sum_{i=0}^{m(n)-1} i + m(n) (c(n) + 1) [ m(n) C + c(n) / 2 ]$

	$= C^2 m(n) (m(n) - 1) (2 m(n) - 1) / 6 + C (C - 1) m(n) (m(n) - 1) / 4 + m(n) (c(n) + 1) [ m(n) C + c(n) / 2 ]$

- $\sum_{i=0}^n alpha_{c_i}$

	$= \sum_{i=0}^{m(n)-1} \sum_{j=0}^{C-1} alpha_{c_j} + \sum_{j=0}^{c(n)} alpha_{c_j}$

	$= \sum_{i=0}^{m(n)-1} A + A(c(n))$

	$= m(n) * A + A(c(n))$

- $\sum_{i=0}^n i alpha_{c_i}$

	$= \sum_{i=0}^{m(n)-1} \sum_{j=0}^{C-1} (i C + j) alpha_{c_j} + \sum_{j=0}^{c(n)} (m(n) C + j) alpha_{c_j}$

	$= \sum_{i=0}^{m(n)-1} [ i C A + X ] + m(n) C A(c(n)) + \sum_{j=0}^{c(n)} j \alpha_{c_j}$

	$= C A \sum_{i=0}^{m(n)-1} i + X \sum_{i=0}^{m(n)-1} 1 + m(n) C A(c(n)) + \sum_{j=0}^{c(n)} j \alpha_{c_j}$

	$= C A m(n) (m(n) - 1) / 2 + m(n) X + m(n) C A(c(n)) + X(c(n))$

- $\sum_{i=0}^n i^2 alpha_{c_i}$

	$= \sum_{i=0}^{m(n)-1} \sum_{j=0}^{C-1} (i C + j)^2 alpha_{c_j} + \sum_{j=0}^{c(n)} (m(n) C + j)^2 alpha_{c_j}$

	$= \sum_{i=0}^{m(n)-1} \sum_{j=0}^{C-1} (i^2 C^2 + 2 i C j + j^2) alpha_{c_j} + \sum_{j=0}^{c(n)} (m(n)^2 C^2 + 2 m(n) C j + j^2) alpha_{c_j}$

	$= \sum_{i=0}^{m(n)-1} [i^2 C^2 A + 2 i C X + U ] + m(n)^2 C^2 A(c(n)) + 2 m(n) C X(c(n)) + U(c(n))$

	$= C^2 A \sum_{i=0}^{m(n)-1} i^2 + 2 C X \sum_{i=0}^{m(n)-1} i + U m(n) + m(n)^2 C^2 A(c(n)) + 2 m(n) C X(c(n)) + U(c(n))$

	$= C^2 A m(n) (m(n) - 1) (2 m(n) - 1) / 6 + C X m(n) (m(n) - 1) + U m(n) + m(n)^2 C^2 A(c(n)) + 2 m(n) C X(c(n)) + U(c(n))$

- $\sum_{i=0}^n m(i) alpha_{c_i}$

	$= \sum_{i=0}^{m(n)-1} \sum_{j=0}^{C-1} i alpha_{c_j} + \sum_{j=0}^{c(n)} m(n) alpha_{c_j}$

	$= \sum_{i=0}^{m(n)-1} i A + m(n) A(c(n))$

	$= A m(n) (m(n) - 1) / 2 + m(n) A(c(n))$

- $\sum_{i=0}^n i m(i) alpha_{c_i}$

	$= \sum_{i=0}^{m(n)-1} \sum_{j=0}^{C-1} (i C + j) i alpha_{c_j} + \sum_{j=0}^{c(n)} (m(n) C + j) m(n) alpha_{c_j}$

	$= \sum_{i=0}^{m(n)-1} i (i C A + X) + m(n)^2 C A(c(n)) + m(n) X(c(n))$

	$= C A \sum_{i=0}^{m(n)-1} i^2 + X \sum_{i=0}^{m(n)-1} i + m(n)^2 C A(c(n)) + m(n) X(c(n))$

	$= C A m(n) (m(n) - 1) (2 m(n) - 1) / 6 + X m(n) (m(n) - 1) / 2 + m(n)^2 C A(c(n)) + m(n) X(c(n))$

- $\sum_{i=0}^n A(i)$

	$= \sum_{i=0}^n \sum_{j=0}^{i} \alpha_{c_j}$

	$= \sum_{i=0}^n (n - i + 1) \alpha_{c_i}$

	$= (n + 1) \sum_{i=0}^n \alpha_{c_i} - \sum_{i=0}^n i \alpha_{c_i}$

	$= (n + 1) [ m(n) * A + A(c(n)) ] - [ C A m(n) (m(n) - 1) / 2 + m(n) X + m(n) C A(c(n)) + X(c(n)) ]$

	$= m(n) A [(n + 1) - C (m(n) - 1) / 2 ] + A(c(n)) [n + 1 - m(n) * C] - m(n) X - X(c(n))$

	$= m(n) A [(n + 1) - C (m(n) - 1) / 2 ] + A(c(n)) (c(n) + 1) - m(n) X - X(c(n))$

	$= m(n) A [C m(n) + c(n) + 1 - C m(n) / 2 + C / 2 ] + A(c(n)) (c(n) + 1) - m(n) X - X(c(n))$

	$= m(n) A [C m(n) / 2 + c(n) + 1 + C / 2 ] + A(c(n)) (c(n) + 1) - m(n) X - X(c(n))$

	$= m(n) A [C (m(n) + 1) / 2 + c(n) + 1 ] + A(c(n)) (c(n) + 1) - m(n) X - X(c(n))$

- $\sum_{i=0}^{c(n)} A(i)$

	$= m(c(n)) A [C (m(c(n)) + 1) / 2 + c(c(n)) + 1 ] + A(c(c(n))) (c(c(n)) + 1) - m(c(n)) X - X(c(c(n)))$

	$= 0 * A [C (0 + 1) / 2 + c(n) + 1 ] + A(c(n)) (c(n) + 1) - 0 * X - X(c(n))$

	$= A(c(n)) (c(n) + 1) - X(c(n))$

- $\sum_{i=0}^{C-1} A(i)$

	$= A C - X$

- $\sum_{i=0}^{n} A(c(i))$

	$= \sum_{i=0}^{m(n)-1} \sum_{j=0}^{C-1} A(c(i C + j)) + \sum_{j=0}^{c(n)} A(c(m(n) C + j))$

	$= \sum_{i=0}^{m(n)-1} \sum_{j=0}^{C-1} A(j) + \sum_{j=0}^{c(n)} A(j)$

	$= \sum_{i=0}^{m(n)-1} A C - X + A(c(n)) (c(n) + 1) - X(c(n))$

	$= (A C - X) m(n) + A(c(n)) (c(n) + 1) - X(c(n))$

- $\sum_{i=0}^{n} m(i) A(c(i))$

	$= \sum_{i=0}^{m(n)-1} i \sum_{j=0}^{C-1} A(c(i C + j)) + \sum_{j=0}^{c(n)} m(n) A(c(m(n) C + j))$

	$= \sum_{i=0}^{m(n)-1} i \sum_{j=0}^{C-1} A(j) + m(n) \sum_{j=0}^{c(n)} A(j)$

	$= \sum_{i=0}^{m(n)-1} i (A C - X) + m(n) A(c(n)) (c(n) + 1) - m(n) X(c(n))$

	$= (A C - X) m(n) (m(n) - 1) / 2  + m(n) A(c(n)) (c(n) + 1) - m(n) X(c(n))$
	
- $\sum_{i=0}^n m(i) A(i)$
	$= \sum_{i=0}^{m(n)-1} i \sum_{j=0}^{C-1} A(i C + j)
		+ m(n) \sum_{j=0}^{c(n)} A(m(n) C + j)$

	$= \sum_{i=0}^{m(n)-1} i \sum_{j=0}^{C-1} i A + A(j)
		+ m(n) \sum_{j=0}^{c(n)} m(n) A + A(j)$

	$= \sum_{i=0}^{m(n)-1} i [i A C + \sum_{j=0}^{C-1} A(j) ]
		+ m(n) [m(n) A (c(n) + 1) + \sum_{j=0}^{c(n)} A(j) ]$

	$= \sum_{i=0}^{m(n)-1} i [i A C + A C - X ]
		+ m(n) [m(n) A (c(n) + 1) + A(c(n)) (c(n) + 1) - X(c(n)) ]$

	$= \sum_{i=0}^{m(n)-1} i [(i+1) A C - X ]
		+ A m(n)^2 (c(n) + 1) +
		+ A(c(n)) m(n) (c(n) + 1)
		- X(c(n)) m(n) $

	$= A C \sum_{i=0}^{m(n)-1} i^2 + \sum_{i=0}^{m(n)-1} i]
		- X \sum_{i=0}^{m(n)-1} i
		+ A m(n)^2 (c(n) + 1) +
		+ A(c(n)) m(n) (c(n) + 1)
		- X(c(n)) m(n) $

	$= A C [ m(n) (m(n) - 1) (2 m(n) - 1) / 6 + m(n) (m(n) - 1) / 2 ]
		- X m(n) (m(n) - 1) / 2
		+ A m(n)^2 (c(n) + 1) +
		+ A(c(n)) m(n) (c(n) + 1)
		- X(c(n)) m(n) $

	$= A C m(n) (m(n) - 1)  / 6 [ (2 m(n) - 1) + 3 ]
		- X m(n) (m(n) - 1) / 2
		+ A m(n)^2 (c(n) + 1) +
		+ A(c(n)) m(n) (c(n) + 1)
		- X(c(n)) m(n) $

	$= A C m(n) (m(n) - 1)  / 6 [ 2 m(n) + 2 ]
		- X m(n) (m(n) - 1) / 2
		+ A m(n)^2 (c(n) + 1) +
		+ A(c(n)) m(n) (c(n) + 1)
		- X(c(n)) m(n) $

	$= A C m(n) (m(n) - 1) (m(n) + 1) / 3 
		- X m(n) (m(n) - 1) / 2
		+ A m(n)^2 (c(n) + 1) +
		+ A(c(n)) m(n) (c(n) + 1)
		- X(c(n)) m(n) $

	$= A m(n) [ C (m(n) - 1) (m(n) + 1) / 3 + m(n) (c(n) + 1) ]
		- X m(n) (m(n) - 1) / 2
		+ A(c(n)) m(n) (c(n) + 1)
		- X(c(n)) m(n) $

	$= A m(n) / 3 [ C m(n)^2 + 3 m(n) (c(n) + 1) - C ]
		- X m(n) (m(n) - 1) / 2
		+ A(c(n)) m(n) (c(n) + 1)
		- X(c(n)) m(n) $

## The final estimations

From there we can reverse the previous definitions:

- $d_n$

	$= d_0 + \sum_{i=1}^n \Delta_i$

	$= d_0 - \Delta_0 + \sum_{i=0}^n \Delta_i$

	$= d_0 - \alpha_0 - m(0) \beta_0 + \sum_{i=0}^n \Delta_i$

	$= d_0 - \alpha_0 + \sum_{i=0}^n \Delta_i$

	$= d_0 - \alpha_0 + \sum_{i=0}^{m(n)-1} \sum_{j=0}^{C-1} \Delta_{i, j}$ + \sum_{j=0}^{c(n)} \Delta_{m, j}$

	$= d_0 - \alpha_0 + \sum_{i=0}^{m(n)-1} \sum_{j=0}^{C-1} [ \alpha_{c_j} + i \beta_{c_j} ] + \sum_{j=0}^{c(n)} [ \alpha_{c_j} + m(n) \beta_{c_j} ]$

	$= d_0 - \alpha_0 + \sum_{i=0}^{m(n)-1} A  + \sum_{i=0}^{m(n)-1} i B + A(c(n)) + m(n) B(c(n))$

	$= d_0 - \alpha_0 + m(n) A  + B \sum_{i=0}^{m(n)-1} i + A(c(n)) + m(n) B(c(n))$

	$= d_0 - \alpha_0 + m(n) A  + B m(n) (m(n) - 1) / 2 + A(c(n)) + m(n) B(c(n))$
	
- $x_n$

	$= x_0 + \sum_{i=1}^n d_i$

	$= x_0 - d_0 + \sum_{i=0}^n d_i$

	$= x_0 - d_0  + \sum_{i=0}^n d_0 - \alpha_0 + m(i) A  + B m(i) (m(i) - 1) / 2 + A(i) + m(i) B(i)$

	$= x_0 + n d_0 - (n + 1) \alpha_0
		+ A \sum_{i=0}^n m(i)
		+ B \sum_{i=0}^n m(i) (m(i) - 1) / 2
		+ \sum_{i=0}^n A(c(i))
		+ \sum_{i=0}^n m(i) B(c(i))$

	$= x_0 + n d_0 - (n + 1) \alpha_0
		+ A \sum_{i=0}^n m(i)
		+ B \sum_{i=0}^n m(i) (m(i) - 1) / 2
		+ (A C - X) m(n) + A(c(n)) (c(n) + 1) - X(c(n))
		+ (B C - Y) m(n) (m(n) - 1) / 2 + m(n) B(c(n)) (c(n) + 1) - m(n) Y(c(n))$

	$= x_0 + n d_0 - (n + 1) \alpha_0
		+ A [ C m(n) (m(n) - 1) / 2 + c(n) m(n) ]
		+ B / 2 [ m(n) (m(n) - 1) ( C (m(n) - 2) / 3 + c(n) ) ]
		+ A C m(n)
		- X m(n)
		+ A(c(n)) (c(n) + 1)
		- X(c(n))
		+ B C m(n) (m(n) - 1) / 2
		- Y m(n) (m(n) - 1) / 2
		+ B(c(n)) m(n) (c(n) + 1)
		- Y(c(n)) m(n)$

	$= x_0 + n d_0 - (n + 1) \alpha_0
		+ A / 2 [ C m(n) (m(n) - 1) + 2 c(n) m(n) + 2 C m(n)]
		+ B m(n) (m(n) - 1) / 2 [ ( C (m(n) - 2) / 3 + c(n) ) + C ]
		- X m(n)
		- Y m(n) (m(n) - 1) / 2
		+ A(c(n)) (c(n) + 1)
		- X(c(n))
		+ B(c(n)) m(n) (c(n) + 1)
		- Y(c(n)) m(n)$

	$= x_0 + n d_0 - (n + 1) \alpha_0
		+ A [ C m(n) (m(n) + 1) / 2  + c(n) m(n) ]
		+ B m(n) (m(n) - 1)  / 6 [ C (m(n) + 1) + 3 c(n)]
		- X m(n)
		- Y m(n) (m(n) - 1) / 2
		+ A(c(n)) (c(n) + 1)
		+ B(c(n)) m(n) (c(n) + 1)
		- X(c(n))
		- Y(c(n)) m(n)$

We some information on x_0, d_0, the cycle length C, the initial values $\alpha$ and $\beta$ we can estimate the number of garden plots directly.