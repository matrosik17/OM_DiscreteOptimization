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
    for n in range(n_items + 1):
        if solve_bp_decision(items, n):
            return n

# You should leave function header intact
def solve_bp_search(items: List[float]) -> List[int]:
    n_items = len(items)
    n_bins = solve_bp_evaluation(items)
    bin_indices = [0 for i in range(n_items)]

    for curr_bin_idx in range(1, n_bins + 1):
        for item_idx in range(n_items):
            if bin_indices[item_idx] != 0:
                continue
            # пробуем поместить обьект item_idx в контейнер curr_bin_idx
            bin_indices[item_idx] = curr_bin_idx
            # свободные обьекты
            curr_items = [items[i] for i in range(n_items) if bin_indices[i] == 0]
            # обьекты, уже помещенные в контейнеры
            bins = []
            for bin_idx in range(1, curr_bin_idx + 1):
                weight = sum(items[i] for i in range(n_items) if bin_indices[i] == bin_idx)
                bins.append(weight)
            # формируем текущий набор предметов
            curr_items += bins

            # проверяем оптимальность такого распределения
            n_bins_estimation = solve_bp_evaluation(curr_items)
            if n_bins_estimation != n_bins:
                bin_indices[item_idx] = 0

    return bin_indices


if __name__ == "__main__":
    arr = [0.8, 0.09, 0.4, 0.7]
    print(solve_bp_search(arr))