int: n = 4;
array[1..n] of var 0..n*n: a;

constraint forall(i in 1..n-1)(a[i] < a[i+1]);

constraint
forall(i, j, k, l in 1..n)(
  if i < j /\ k < l /\ (i != k \/ j != l) then
    a[j] - a[i] != a[l] - a[k]
  endif
);

solve minimize a[n];