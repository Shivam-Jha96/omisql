import os
import sys
import platform
import stat

VERSION = os.environ.get("OMNISQL_VERSION", "v0.1.0")
REPO = "Shivam-Jha96/omnisql"

def get_platform():
    system = platform.system().lower()
    if system == "windows":
        return "windows"
    elif system == "darwin":
        return "macos"
    elif system == "linux":
        return "linux"
    else:
        raise RuntimeError(f"Unsupported platform: {system}")

def get_arch():
    machine = platform.machine().lower()
    if machine in ["x86_64", "amd64"]:
        return "x86_64"
    elif machine in ["arm64", "aarch64"]:
        return "aarch64"
    else:
        raise RuntimeError(f"Unsupported architecture: {machine}")

def main():
    plat = get_platform()
    arch = get_arch()
    suffix = "zip" if plat == "windows" else "tar.gz"
    asset_name = f"omnisql-{VERSION}-{arch}-{plat}.{suffix}"
    url = f"https://github.com/{REPO}/releases/download/{VERSION}/{asset_name}"
    
    bin_name = "omnisql.exe" if plat == "windows" else "omnisql"
    bin_dir = os.path.join(os.path.dirname(__file__), "bin")
    
    if not os.path.exists(bin_dir):
        os.makedirs(bin_dir)
        
    bin_path = os.path.join(bin_dir, bin_name)
    
    print(f"Downloading OmniSQL for {plat}-{arch} from {url}...")
    
    # Mocking download for MVP
    try:
        with open(bin_path, "w") as f:
            f.write("#!/usr/bin/env python\nprint('OmniSQL Mock Binary Execution')\n")
        
        # Make executable
        st = os.stat(bin_path)
        os.chmod(bin_path, st.st_mode | stat.S_IEXEC)
        print(f"Successfully installed OmniSQL to {bin_path}")
    except Exception as e:
        print(f"Failed to install OmniSQL: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
