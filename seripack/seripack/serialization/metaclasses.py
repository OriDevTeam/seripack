# Standard Imports
from abc import ABCMeta

# Application modules


# Library modules


PROPERTY_ATTRIBUTE_NAME = "_property"
PACKET_FUNCTIONS_LIST_NAME = "_packet_functions"


class PacketSerializationMeta(type):

    @classmethod
    def __prepare__(mcs, name: str, bases: tuple):
        return super(PacketSerializationMeta, mcs).__prepare__(name, bases)

    def __new__(mcs, name: str, bases: tuple, dct):

        # TODO: This does not take account for multiple inheritance at the same level
        # TODO: this can be complemented by checking the other bases, unsure if __mro__ is
        # TODO: resolved at this time since this should be where it is resolved
        if len(bases) > 0:
            mcs.__determine_property(dct, bases[0])

        return super(PacketSerializationMeta, mcs).__new__(mcs, name, bases, dct)

    @staticmethod
    def __determine_property(dct: dict, base):
        if PACKET_FUNCTIONS_LIST_NAME not in dct:
            dct[PACKET_FUNCTIONS_LIST_NAME] = []

        if hasattr(base, PACKET_FUNCTIONS_LIST_NAME):
            base_functions = getattr(base, PACKET_FUNCTIONS_LIST_NAME)
            for function in base_functions:
                dct[PACKET_FUNCTIONS_LIST_NAME].append(function)

        for attr, val in dct.items():
            if hasattr(val, PROPERTY_ATTRIBUTE_NAME):
                dct[PACKET_FUNCTIONS_LIST_NAME].append(val)


PacketSerializationMetaInterfaceMixin = type('PacketSerializationMetaInterfaceMixin',
                                             (ABCMeta, PacketSerializationMeta), {})
