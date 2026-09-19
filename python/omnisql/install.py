import os
import sys
import platform
import stat
import urllib.request
import tarfile
import zipfile
import shutil

VERSION = os.environ.get("OMNISQL_VERSION", "v1.0.2")
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
    bin_dir = os.path.join(os.path.expanduser("~"), ".omnisql", "bin", VERSION)
    
    if not os.path.exists(bin_dir):
        os.makedirs(bin_dir)
        
    bin_path = os.path.join(bin_dir, bin_name)
    
    print(f"Downloading OmniSQL for {plat}-{arch} from {url}...")
    
    tmp_path = os.path.join(bin_dir, f"temp_archive.{suffix}")
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})
        with urllib.request.urlopen(req) as response, open(tmp_path, 'wb') as out_file:
            shutil.copyfileobj(response, out_file)
        
        if suffix == "zip":
            with zipfile.ZipFile(tmp_path, 'r') as zip_ref:
                zip_ref.extract(bin_name, path=bin_dir)
        else:
            with tarfile.open(tmp_path, 'r:gz') as tar_ref:
                tar_ref.extract(bin_name, path=bin_dir)
                
        os.remove(tmp_path)
        
        # Make executable
        os.chmod(bin_path, 0o755)
        print(f"Successfully installed OmniSQL to {bin_path}")
    except Exception as e:
        if os.path.exists(tmp_path):
            os.remove(tmp_path)
        print(f"Failed to install OmniSQL: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
