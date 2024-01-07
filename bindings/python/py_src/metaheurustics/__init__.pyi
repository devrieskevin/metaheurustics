from typing import Callable, List

from metaheurustics.typing import (
    IndividualProtocol,
    MutatorProtocol,
    RecombinatorProtocol,
)

class BitFlip:
    def __init__(
        self,
        probability: float,
        max_bit_count: int,
        min_value: int,
        max_value: int,
    ) -> None:
        pass

class ReplaceWorst:
    def __init__(self, replacement_rate: float) -> None:
        pass

class LinearRanking:
    def __init__(self, s: float) -> None:
        pass

class OnePoint:
    def __init__(self) -> None:
        pass

class SmallRng:
    def __init__(self, seed: int | None) -> None:
        pass

class Individual:
    def __init__(self, individual: IndividualProtocol) -> None:
        pass

class IndividualMutator:
    def __init__(self, mutator: MutatorProtocol) -> None:
        pass
    def mutate(
        self,
        rng: SmallRng,
        individual: IndividualProtocol,
    ) -> IndividualProtocol:
        pass

class IndividualRecombinator:
    def __init__(self, recombinator: RecombinatorProtocol) -> None:
        pass
    def recombine(
        self,
        rng: SmallRng,
        parents: List[IndividualProtocol],
    ) -> List[IndividualProtocol]:
        pass

class Solver:
    def __init__(
        self,
        rng: SmallRng,
        parent_selector: LinearRanking,
        recombinator: RecombinatorProtocol,
        mutator: MutatorProtocol,
        survivor_selector: ReplaceWorst,
        evaluator: Callable[[IndividualProtocol], object],
        initializer: Callable[[SmallRng, int], IndividualProtocol],
    ) -> None:
        pass
    def solve(self, population_size: int, number_iterations: int) -> List[object]:
        pass
