# Standard Imports
from typing import List
from abc import ABC, abstractmethod


# Library Imports


# External Imports


class SerializerABC(ABC):
	
	@staticmethod
	@abstractmethod
	def serialize(raw_message: bytes):
		pass
	
	@staticmethod
	@abstractmethod
	def deserialize(packed_message: bytes):
		pass


class SerializationSetInterface(ABC):
	
	@property
	@abstractmethod
	def serializations(self) -> List[SerializerABC]:
		pass
	
