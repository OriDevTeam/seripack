# Standard Imports
from typing import Type

# Library Imports
from seripack.packet_set.interfaces import SetABC
from seripack.packet.abstractions import PacketABC

# External Imports


def packet_set(packet_set_type: Type[SetABC]):
	
	def decorator(packet_type: Type[PacketABC]):
		
		if not hasattr(packet_set_type, 'packet_types'):
			packet_set_type.packet_types = []
		
		if packet_type not in packet_set_type.packet_types:
			packet_set_type.packet_types.append(packet_type)
		
		# @wraps(packet_type)
		# def func(*args, **kwargs):
		# 	return packet_type
		
		return packet_type
	
	return decorator
