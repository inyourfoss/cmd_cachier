#!/usr/bin/env python

import tomllib
import os

from datetime import datetime, date
from string import Template


with open("Cargo.toml", "rb") as f:
    proj_data = tomllib.load(f)

with open("default.nix.templ", "r") as f:
    buildfile = f.read()

metadata = {
    "version": proj_data['package']['version'],
    "timestamp": datetime.now().replace(microsecond=0),
    "date": date.today(),
    "commithash": os.popen('git rev-parse --verify HEAD').read().strip()
}

t = Template(buildfile)

result = t.safe_substitute(metadata)
print(result)

