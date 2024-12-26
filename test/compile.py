import subprocess

def compile(path):
    output = subprocess.check_output(f"cargo run -- {path}", shell=True)
    return output.decode("utf-8")
