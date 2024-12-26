import os
from compile import compile

for file in os.listdir("source"):
    output = compile(f"source/{file}")
    expected = open(f"output/{file}").read()
    if output == expected:
        print(f"[{file}]: OK")
    else:
        print(f"[{file}]: FAIL")
