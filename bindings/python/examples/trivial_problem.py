import metaheurustics as mh

from typing import Callable

if __name__ == "__main__":
    problem: Callable[[dict], int] = lambda parameters: parameters["value"]

    # Algorithm draft
    algorithm = {
        "parameters": {
            "value": {
                "type": int,
                "mutation": mh.BitFlip(0.1, 10, 0, 100),
                "recombination": mh.OnePoint(),
            }
        },
        "parent_selection": mh.LinearRanking(1.5),
        "survivor_selection": mh.ReplaceWorst(0.5),
        "problem": problem,
    }

    print(type([]))
