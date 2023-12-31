import metaheurustics as mh

from typing import List

class MyIndividual:
    def __init__(self, value: int):
        self.value = value


class MyIndividualMutator:
    value_mutator: mh.BitFlip

    def __init__(self):
        self.value_mutator = mh.BitFlip(.5, 10, 0, 2**31 - 1)

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

if __name__ == "__main__":
    a = mh.Individual(MyIndividual(10))
    b = mh.Individual(MyIndividual(20))
    rng = mh.SmallRng(5)

    mutator = mh.IndividualMutator(MyIndividualMutator())
    recombinator = mh.IndividualRecombinator(MyIndividualRecombinator())

    children = recombinator.recombine(rng, [a, b])

    print(children[0].individual.value, children[1].individual.value)

    for child in children:
        mutator.mutate(rng, child)

    print(children[0].individual.value, children[1].individual.value)
