import metaheurustics as mh

from typing import Callable

if __name__ == "__main__":
    problem: Callable[[dict], int] = lambda parameters: parameters["value"]

    # Algorithm draft
    algorithm = {
        "parameters": {
            "value": {
                "type": int,
                "mutation": "bit-flip",
                "recombination": "one-point",
            }
        },
        "parent_selection": "linear_ranking",
        "survivor_selection": "round_robin_tournament",
        "problem": problem,
    }

    print(mh.type_name(float))