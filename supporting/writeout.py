#! /usr/bin/env nix-shell
#! nix-shell -i python3 -p python3 withPackages(ps: [ ps.pandas ps.numpy ps.pyarrow])

# nix-shell -p "python3.withPackages(ps: [ ps.pandas ps.numpy ps.pyarrow ps.yapf])"

from pyarrow import json as paj
import json as js
import numpy as np
import pandas as pd
import pyarrow as pa
import pyarrow.parquet as pq

print("Start ....")

x = np.random.randn(10000)

df = pd.DataFrame({'one': x, 'two': x + 5, 'three': x > 0.1}, )

table = pa.Table.from_pandas(df)
pq.write_table(table, 'example.parquet')

with open("exemplary.json", "w") as f:
    js.dump(
        dict(x=list(x),
            y=[dict(a=1, b=5, c=False) for i in list(x)]
        ), f)

table = paj.read_json(f"exemplary.json")
pq.write_table(table, 'example_nested.parquet')
print("Stop ....")

