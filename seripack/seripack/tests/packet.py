# Standard Imports
from dataclasses import dataclass
from unittest import TestCase

# Library Imports
from seripack.packet.decorators import packet
from seripack.serialization import Serialization
from seripack.serialization.decorators import route
from seripack.packet.abstractions import PacketABC
from seripack.serialization.serializers.pickle import Pickle5

# External Imports
from seripack.packet.routing import Router
from seripack.serialization.serializers.blosc import Blosc


class TestPacketCreation(TestCase):

    def test_packet(self):

        class TestSet(Serialization):
            serializers = [Pickle5, Blosc]

        @packet(0x1, TestSet)
        @dataclass(order=True, slots=True)
        class TestPacket(PacketABC):
            ...

        class TestRouter(Router):
            packets = [TestPacket]

        @route(TestPacket)
        def test_packet_routing(test: PacketABC):
            return True

        test_set = TestSet()
        test_packet = TestPacket()

        test_set.resolve_route(test_packet)

        self.assertTrue(test_packet)


