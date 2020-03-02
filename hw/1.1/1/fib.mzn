int: n = 30;
array[1..n] of var int: fib;

constraint fib[1] = 1;
constraint fib[2] = 1;
constraint forall(i in 3..n) (
    fib[i] = fib[i-1] + fib[i-2]
);

solve satisfy;