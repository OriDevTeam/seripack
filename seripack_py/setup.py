# Standard Imports
from setuptools import setup

# External Imports
from setuptools_rust import RustExtension, Binding


setup(
    name="seripack",
    version="0.1",
    rust_extensions=[
        RustExtension(
            "seripack",
            binding=Binding.PyO3,
            args=["--profile", "release-lto"],
        )
    ],
    packages=["seripack"],
    zip_safe=False,
)

