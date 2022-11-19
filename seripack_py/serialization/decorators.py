# Standard Imports
from functools import wraps
from typing import Type

# Library Imports
from seripack.packet.abstractions import PacketABC
from seripack.serialization.interfaces import SerializationSetInterface
from seripack.serialization.metaclasses import PROPERTY_ATTRIBUTE_NAME

# External Imports


def serialization_set(_type: Type[SerializationSetInterface]):
	def decorator(serialization_type):
		
		if not hasattr(_type, 'serializations'):
			serialization_set.serializations = []
		
		if serialization_type not in _type.serializations:
			_type.serializations.append(serialization_type)
		
		@wraps(serialization_type)
		def wrapper(*args, **kwargs):
			serialization_type(*args, **kwargs)
		
		return wrapper
	
	return decorator


# TODO: Direction argument does not do anything since the packet direction is fixed, decide if instead
# TODO: a packet manager can be implemented (registers which packets go from where to where)
# TODO: Current meaning is: Outgoing from Client to server and Incoming from server to Client
def route(_type: Type[PacketABC]):
	def decorator(func):
		setattr(func, PROPERTY_ATTRIBUTE_NAME, True)
		setattr(func, 'packet', _type)

		return func
	
	return decorator

