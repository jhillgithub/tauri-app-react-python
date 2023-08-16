import os
import sys

path = sys.argv[1]

if path.startswith("~"):
    path = os.path.expanduser(path)

if os.path.isdir(path):
    for file in os.listdir(path):
        if os.path.isfile(os.path.join(path, file)):
            print(file)
else:
    print("Invalid path")