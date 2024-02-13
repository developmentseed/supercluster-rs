# supercluster-rs

[![crates.io version](https://img.shields.io/crates/v/supercluster-rs.svg)](https://crates.io/crates/supercluster-rs)
[![docs.rs docs](https://docs.rs/supercluster-rs/badge.svg)](https://docs.rs/supercluster-rs)

A Rust port of [Supercluster](https://github.com/mapbox/supercluster) for fast hierarchical point clustering.

## Features

- Rust-native port of the [original JavaScript implementation](https://github.com/mapbox/supercluster).
- Built on the efficient zero-copy K-d tree from the [geo-index crate](https://github.com/kylebarron/geo-index).
- Initial Python bindings to efficiently connect to Python via Arrow.

## Drawbacks

- The supercluster algorithm is (currently) tied to the Spherical Mercator coordinate system, so it's most useful for visualization use cases. This may be improved in the future.

## Alternatives

The primary existing alternative to this library is [`supercluster`](https://github.com/chargetrip/supercluster-rust), which was written at about the same time. Below is an overview of the differences as of `supercluster` version 1.0.16.

### Input data flexibility

That library is _deeply tied to GeoJSON_. For example, to load data into the Supercluster object, you [_must_ pass in GeoJSON features](https://docs.rs/supercluster/latest/supercluster/struct.Supercluster.html#method.load).

For better performance, improved memory usage, and more flexibility, this `supercluster-rs` library does not tie itself to GeoJSON. Instead, it operates on positional indexes. So the results of queries return indexes into the input collection. This means you can use any input data structure — not limited to GeoJSON — including a `Vec` of rust objects or a [GeoArrow table](https://github.com/geoarrow/geoarrow-rs).

### "Rust-native" port

That library also is a _direct_ port from JS to Rust with as few changes as possible. For best performance, the original JS library is written in a way that's hard to understand and extend. For example, in JS, all internal data (i.e. tree indexes, positional coordinates, etc) is stored in a JavaScript `Float64Array` and it's up to the library to ensure the offsets within the array are correct.

In contrast, this `supercluster-rs` crate ported code to be more "rust-native" which should allow for safer refactors in the future to add more functionality not present in the original JS library. There are more "new-type" abstractions so the compiler can ensure types are accurate.

### Zero-copy indexes

Essentially, the Supercluster object is a hierarchical HashMap where keys are the integer Web Mercator zoom level and values are a K-D tree index at that zoom level. It's hierarchical: the index at each zoom level stores only the clusters at the following level, not the entire dataset, enabling better scaling.

This library is implemented on top of the [geo-index crate](https://github.com/kylebarron/geo-index), which defines zero-copy spatial indexes for FFI integration with other languages like JavaScript and Python. That enables future work to connect this `supercluster-rs` library to other languages via FFI without serialization.

## Future work

- Support non-web mercator projections.
