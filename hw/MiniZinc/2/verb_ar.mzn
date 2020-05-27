include "alldifferent.mzn";

int: max_len = 5; % максимальная длина слова
set of int: Digits = 0..9;
array [1..max_len] of int: a = [10^(i - 1) | i in 1..max_len];

function var int: to_num(array[int] of var Digits: arr) = let {
  int: n = length(arr);
} in sum(i in 1..n)(a[n - i + 1] * arr[i]);

% первый ребус
% sun + fun = swim
var Digits: s;
var Digits: u;
var Digits: n;
var Digits: f;
var Digits: w;
var Digits: i;
var Digits: m;

constraint to_num([s, u, n]) + to_num([f, u, n]) = to_num([s, w, i, m]);
constraint alldifferent([s, u, n, f, w, i, m]);


% второй ребус
% send + more = money
var Digits: S;
var Digits: e;
var Digits: N;
var Digits: d;
var Digits: M;
var Digits: o;
var Digits: r;
var Digits: y;

constraint to_num([S, e, N, d]) + to_num([M, o, r, e]) = to_num([M, o, N, e, y]);
constraint alldifferent([S, e, N, d, M, o, r, y]);

solve satisfy;

output(["sun + fun = swim\n"]);
output(["\(s)\(u)\(n) + \(f)\(u)\(n) = \(s)\(w)\(i)\(m)\n\n"]);

output(["SeNd + More= MoNey\n"]);
output(["\(S)\(e)\(N)\(d) + \(M)\(o)\(r)\(e)= \(M)\(o)\(N)\(e)\(y)\n"]);