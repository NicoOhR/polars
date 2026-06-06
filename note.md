the problem is almost certain the inconsistency between the `arrow-polars` crate and the
actual pyarrow conventions for a struct array. 

`sink_delta` with a lazy frame calls `write_deltalake` from the deltalake package, 
which calls `RecordBatchReader.from_arrow()` (arro3), which calls `__arrow_c_stream__` on the polars `CollectBatches`

from Polars, `PyCollectBatches.__arrow_c_stream__` calls `export_iterator`, which
triggers `ArrowStreamIterator::next` which it serializes with `ToFfi for StructArray`

`ToFfi::children()` on the outer struct gives back the`info` struct, children being
serialized the same way, then deltalake impots `arrow-rs` and the assertion fires in
`arrow-rs` due to the mismatch in slicing.
