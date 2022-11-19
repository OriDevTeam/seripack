# Standard Imports
from functools import wraps
from typing import Type

# Library Imports
from seripack.packet.abstractions import PacketABC
from seripack.packet_set.interfaces import SetABC

# External Imports


def packet(_id: int, _set: Type[SetABC]):
	
	def decorator(_packet: Type[PacketABC]):
		
		if not hasattr(_set, 'packet_types'):
			_set.packet_types = []
		
		if _packet not in _set.packet_types:
			_set.packet_types.append(_packet)
		
		@wraps
		def func(f_args, f_kwargs):
			return f_args, f_kwargs
		
		return func
	
	return decorator
