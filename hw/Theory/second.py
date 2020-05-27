from typing import List

# Leave the whole “solve_bp_decision” function intact
def solve_bp_decision(items: List[float], n_bins: int) -> bool:
    def able_to_pack(items: List[float], bin_capacities: List[float]) -> bool:
        return items == [] or any(
            able_to_pack(
                items[:-1],
                bin_capacities[:i] + [capacity - items[-1]] + bin_capacities[(i + 1):]
            )
            for i, capacity in enumerate(bin_capacities) if capacity >= items[-1]
        )

    return able_to_pack( sorted(items), [1.0] * n_bins )

# To test this, one can run:
# solve_bp_decision([0.8, 0.09, 0.4, 0.7], 2)
# solve_bp_decision([0.8, 0.09, 0.4, 0.7], 3)


# You should leave function header intact
def solve_bp_evaluation(items: List[float]) -> int:
    n_items = len(items)
    for n in range(n_items):
        if solve_bp_decision(items, n):
            return n

# You should leave function header intact
def solve_bp_search(items: List[float]) -> List[int]:
    #   … your code here …
    pass