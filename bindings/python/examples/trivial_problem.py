import metaheurustics as mh
import random

from typing import List

class MyIndividual:
    fitness: float
    age: int

    def __init__(self, value: int):
        self.value = value

    def get_fitness(self) -> float:
        return self.fitness

    def set_fitness(self, fitness: float) -> None:
        self.fitness = fitness

    def get_age(self) -> int:
        return self.age

    def set_age(self, age: int) -> None:
        self.age = age

class MyIndividualMutator:
    value_mutator: mh.BitFlip

    def __init__(self):
        self.value_mutator = mh.BitFlip(.5, 10, 0, 100)

    def mutate(self, rng: mh.SmallRng, individual: MyIndividual) -> MyIndividual:
        individual.value = self.value_mutator.mutate(rng, individual.value)
        return individual

class MyIndividualRecombinator:
    value_recombinator: mh.OnePoint

    def __init__(self):
        self.value_recombinator = mh.OnePoint()

    def recombine(self, rng: mh.SmallRng, parents: List[MyIndividual]) -> List[MyIndividual]:
        children = self.value_recombinator.recombine(rng,[parent.value for parent in parents])
        return [MyIndividual(child) for child in children]

def evaluate(individual: MyIndividual) -> float:
    return -(individual.value - 50.0)**2

def initialize_population(_rng: mh.SmallRng, size: int) -> List[MyIndividual]:
    return [MyIndividual(random.randint(0, 100)) for _ in range(size)]

if __name__ == "__main__":
    my_rng = mh.SmallRng(None)
    solver = mh.Solver(
        my_rng,
        mh.LinearRanking(1.5),
        MyIndividualRecombinator(),
        MyIndividualMutator(),
        mh.ReplaceWorst(.1),
        evaluate,
        initialize_population
    )
    results = solver.solve(100, 100)

    print(results[0].individual.value)
