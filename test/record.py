import os
from compile import compile

for file in os.listdir("source"):
    output = compile(f"source/{file}")
    with open(f"output/{file}", "w") as output_file:
        output_file.write(output)
