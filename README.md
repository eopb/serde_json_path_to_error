# serde_json_path_to_error


[![License](https://img.shields.io/crates/l/serde_json_path_to_error.svg)](https://crates.io/crates/serde_json_path_to_error)
[![Latest version](https://img.shields.io/crates/v/serde_json_path_to_error.svg)](https://crates.io/crates/serde_json_path_to_error)
[![Latest Docs](https://docs.rs/serde_json_path_to_error/badge.svg)](https://docs.rs/serde_json_path_to_error/)
[![downloads-badge](https://img.shields.io/crates/d/serde_json_path_to_error.svg)](https://crates.io/crates/serde_json_path_to_error)

[API docs](https://docs.rs/serde_path_to_error/)

A drop in replacement for [serde_json] where errors are enriched with [serde_path_to_error] by default.

This is a better default when you need to debug why serialization or deserialization failed.
Paths are particularly helpful when your schema is large or when it's difficault to see the raw data that cause an error.

You may want to avoid this crate if you've benchmarked your system and found serialization or deserialization to be a bottleneck.

## Migrating from [serde_json]

To enrich your errors simply replace your dependancy on [serde_json] with one on serde_json_path_to_error.

```diff
- serde_json = "1.0"
+ serde_json = { package = "serde_json_path_to_error", version = "0.1" }
```

Alternatively, you can add serde_json_path_to_error as a regular dependancy.

```text
# cargo add serde_json_path_to_error 
```

And rename the crate in your crate root to get the same API as [serde_json].

```rust
extern crate serde_json_path_to_error as serde_json;
```


In most cases your project should continue to compile.
Your errors will now be enriched with additional context showing the path to serialization and deserialization failures.

## Examples

```rust
// the rename trick shown above
extern crate serde_json_path_to_error as serde_json;

# use std::collections::BTreeMap as Map;
# use serde::Deserialize;
#[derive(Deserialize)]
struct Package {
    name: String,
    dependencies: Map<String, Dependency>,
}

#[derive(Deserialize)]
struct Dependency {
    version: String,
}

fn main() {
    let j = r#"{
        "name": "demo",
        "dependencies": {
            "serde": {
                "version": 1
            }
        }
    }"#;


    // Uses the enriched version from [serde_json_path_to_error] but with the exact same API
    // you've come to expect from [serde_json]
    let result: Result<Package, _> = serde_json::from_str(j);

    match result {
        Ok(_) => panic!("expected a type error"),
        Err(err) => {
            // You get the error including the path as a default
            assert_eq!(
              err.to_string(),
              "dependencies.serde.version: invalid type: integer `1`, expected a string at line 5 column 28",
            );
            // You can get just the path
            assert_eq!(
              err.path().to_string(),
              "dependencies.serde.version",
            );
            // Or just the original serde_json error
            assert_eq!(
              err.into_inner().to_string(),
              "invalid type: integer `1`, expected a string at line 5 column 28",
            );
        }
    }
}
```

## Caveats



[serde_json]: https://docs.rs/serde_json/latest/serde_json/
[serde_path_to_error]: https://docs.rs/serde_json/latest/serde_path_to_error/