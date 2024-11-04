#!/usr/bin/env python3
import sys

if sys.version_info < (3, 11):
    sys.exit("Python 3.11 or greater is needed to run this script.")


import tomllib
import os

from glob import glob
from pathlib import Path
from datetime import datetime, date
from string import Template

files = sys.argv[1:]

with open("crate/Cargo.toml", "rb") as f:
    proj_data = tomllib.load(f)

metadata = {
    "version": proj_data['package']['version'],
    "timestamp": datetime.now().replace(microsecond=0),
    "date": date.today(),
    "commithash": os.popen('git rev-parse --verify HEAD').read().strip()
}

for file in glob("**/*.templ", recursive=True):
    with open(file, "r") as f:
        docfile = f.read()

    outfile = os.path.splitext(file)[0]

    with open(outfile, "w") as o:
        t = Template(docfile)
        result = t.safe_substitute(metadata)
        print(f"Untemplating {file} to {outfile}...")
        o.write(result)



