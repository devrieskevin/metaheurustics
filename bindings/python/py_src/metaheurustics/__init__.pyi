from typing import List


class BitFlip:
    def __init__(
        self,
        probability: float,
        max_bit_count: int,
        min_value: int,
        max_value: int
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
    def __init__(self, seed: int|None) -> None:
        pass

class Individual:
    def __init__(self, individual: object) -> None:
        pass

class IndividualMutator:
    def __init__(self, mutator: object) -> None:
        pass

    def mutate(self, rng: SmallRng, individual: object) -> object:
        pass

class IndividualRecombinator:
    def __init__(self, recombinator: object) -> None:
        pass

    def recombine(
        self,
        rng: SmallRng,
        parents: List[object]
    ) -> object:
        pass
