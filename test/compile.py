import subprocess
import os

DEV_NULL = open(os.devnull, "w")

def compile(path):
    output = subprocess.check_output(f"cargo run -- {path}", shell=True, stderr=DEV_NULL)
    return output.decode("utf-8")
