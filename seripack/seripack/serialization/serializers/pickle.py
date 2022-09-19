# Standard Imports
import pickle

# Library Imports
from seripack.packet.abstractions import PacketABC
from seripack.serialization.interfaces import SerializerABC

# External Imports


class Pickle5(SerializerABC):
	
	@staticmethod
	def serialize(packet: PacketABC) -> bytes:
		return pickle.dumps(protocol=5, obj=packet)

	@staticmethod
	def deserialize(packed_message: bytes) -> PacketABC:
		return pickle.loads(packed_message)

