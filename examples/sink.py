import tempfile
import polars as pl

schema = {"name": pl.String, "info": pl.Struct({"city": pl.String})}

lf = pl.LazyFrame(
    {"name": ["Alice", "Bob"], "info": [{"city": "Paris"}, None]},
    schema=schema,
)

with tempfile.TemporaryDirectory() as tmp:
    lf.sink_delta(f"{tmp}/test", mode="append")
