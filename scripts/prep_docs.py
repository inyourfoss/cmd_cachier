#!/usr/bin/env python
import tomllib

from datetime import datetime, date
from string import Template

with open("Cargo.toml", "rb") as f:
    proj_data = tomllib.load(f)

with open("templates/templ_cmd_cachier.1.adoc", "r") as f:
    docfile = f.read()

metadata = {
    "version": proj_data['package']['version'],
    "timestamp": datetime.now().replace(microsecond=0),
    "date": date.today()
}

t = Template(docfile)

result = t.safe_substitute(metadata)
print(result)

