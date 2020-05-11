int: n_items;
int: weight_limit;

array[1..n_items] of int: value;
array[1..n_items] of int: weight;

array[1..n_items] of var 0..1: x;
var int: total_value = sum(i in 1..n_items) (x[i] * value[i]);
var int: total_weight = sum(i in 1..n_items) (x[i] * weight[i]);

constraint total_weight <= weight_limit;
solve maximize total_value;

output [show(total_weight)];
output [show(total_value)];
