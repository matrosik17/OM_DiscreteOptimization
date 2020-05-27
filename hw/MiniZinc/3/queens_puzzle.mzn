include "alldifferent.mzn";

int: n = 8;
set of int: Field = 1..n;
array[1..n] of var Field: queens;

% все ферзи стоят на разных вертикалях
constraint alldifferent(queens);

% все ферзи стоят на разных диагоналях
% диагонали задается уравнениями:
% y1(x) = (y0 - x0) + x
% y2(x) = (y0 + x0) - x
% если диагонали пересекают прямую y = 0 в разных точках, то они различны
constraint alldifferent([x0 - queens[x0] | x0 in 1..n]);
constraint alldifferent([queens[x0] + x0 | x0 in 1..n]);

solve satisfy;