# Standard Imports

# Library Imports

# External Imports


class Packet:

    def serialize(self) -> bytes:
        ...

    def deserialize(self) -> 'Packet':
        ...

