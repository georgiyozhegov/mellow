import os

for name in os.listdir("source"):
    source = open(f"source/{name}").read()
    output = open(f"output/{name}").read()
    print(f"=== {name} ===")
    print(source)
    print(f"--- output ---")
    print(output)
