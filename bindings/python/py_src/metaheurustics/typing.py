from typing import List, Protocol

from metaheurustics import SmallRng


class IndividualProtocol(Protocol):
    """A protocol describing the methods that an individual must implement.

    Args:
        Protocol (_type_): _description_
    """

    def get_fitness(self) -> object:
        """Returns the fitness of the individual.

        Returns:
            object: The fitness of the individual.
        """

    def set_fitness(self, fitness: object) -> None:
        """Sets the fitness of the individual.

        Args:
            fitness (object): The fitness of the individual.
        """

    def get_age(self) -> int:
        """Returns the age of the individual.

        Returns:
            int: The age of the individual.
        """

    def set_age(self, age: int) -> None:
        """Sets the age of the individual.

        Args:
            age (int): The age of the individual.
        """


class MutatorProtocol(Protocol):
    """A protocol describing the methods that a mutator must implement.

    Args:
        Protocol (_type_): _description_
    """

    def mutate(self, rng: SmallRng, individual: IndividualProtocol):
        """Mutates an individual.

        Args:
            rng (object): A random number generator.
            individual (IndividualProtocol): The individual to mutate.

        Returns:
            IndividualProtocol: The mutated individual.
        """


class RecombinatorProtocol(Protocol):
    """A protocol describing the methods that a recombinator must implement.

    Args:
        Protocol (_type_): _description_
    """

    def recombine(
        self,
        rng: SmallRng,
        parents: List[IndividualProtocol],
    ) -> List[IndividualProtocol]:
        """Recombines a list of individuals.

        Args:
            rng (object): A random number generator.
            parents (List[IndividualProtocol]): The individuals to recombine.

        Returns:
            List[IndividualProtocol]: The recombined individuals.
        """
