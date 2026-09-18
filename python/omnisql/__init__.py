import os
import sys
import subprocess
import platform

def main():
    plat = platform.system().lower()
    bin_name = "omnisql.exe" if plat == "windows" else "omnisql"
    bin_path = os.path.join(os.path.dirname(__file__), "bin", bin_name)
    
    if not os.path.exists(bin_path):
        print(f"OmniSQL binary not found at {bin_path}.")
        print("Please ensure the package was installed correctly.")
        sys.exit(1)
        
    args = sys.argv[1:]
    
    # Run the binary
    try:
        if plat == "windows":
            # On windows, since it's a mocked Python script for now, run with sys.executable
            # In real scenario it would be a real exe, so we just run the bin_path
            with open(bin_path, "r") as f:
                content = f.read()
            if "python" in content.split("\n")[0]:
                result = subprocess.run([sys.executable, bin_path] + args)
            else:
                result = subprocess.run([bin_path] + args)
        else:
            result = subprocess.run([bin_path] + args)
        sys.exit(result.returncode)
    except Exception as e:
        print(f"Failed to execute OmniSQL: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
