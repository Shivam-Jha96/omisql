import os
import sys
import platform
import subprocess
from setuptools import setup, find_packages
from setuptools.command.build_py import build_py

setup(
    name="omnisql",
    version="1.0.4",
    packages=find_packages(),
)
