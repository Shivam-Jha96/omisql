import os
import sys
import subprocess
import platform

def main():
    plat = platform.system().lower()
    bin_name = "omnisql.exe" if plat == "windows" else "omnisql"
    
    # Import VERSION to use in path
    from .install import VERSION
    
    bin_path = os.path.join(os.path.expanduser("~"), ".omnisql", "bin", VERSION, bin_name)
    
    args = sys.argv[1:]
    if "-V" in args or "--version" in args:
        print(f"omnisql {VERSION.lstrip('v')}")
        sys.exit(0)

    if not os.path.exists(bin_path):
        print(f"OmniSQL {VERSION} binary not found locally. Downloading the correct version for your system...")
        from .install import main as install_main
        try:
            install_main()
        except Exception as e:
            print(f"Failed to install OmniSQL: {e}")
            sys.exit(1)
            
        if not os.path.exists(bin_path):
            print(f"OmniSQL binary is still missing at {bin_path} after installation attempt.")
            sys.exit(1)
        
    # Run the binary
    try:
        result = subprocess.run([bin_path] + args)
        sys.exit(result.returncode)
    except Exception as e:
        print(f"Failed to execute OmniSQL: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
