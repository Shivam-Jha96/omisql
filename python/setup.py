import os
import sys
import platform
import subprocess
from setuptools import setup, find_packages
from setuptools.command.build_py import build_py

class CustomBuildCommand(build_py):
    def run(self):
        # Run the standard build process
        build_py.run(self)
        
        # Then, download the binary
        # In a real package, you might do this in build_ext or a custom install command
        # We will execute the install.py script
        install_script = os.path.join(os.path.dirname(__file__), 'omnisql', 'install.py')
        subprocess.check_call([sys.executable, install_script])

setup(
    name="omnisql",
    version="0.1.0",
    packages=find_packages(),
    cmdclass={
        'build_py': CustomBuildCommand,
    },
    include_package_data=True,
    package_data={'omnisql': ['bin/*']},
)
