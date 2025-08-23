# Zipcodes

[![Docs](https://docs.rs/zipcodes/badge.svg)](https://docs.rs/zipcodes)
[![Crates.io](https://img.shields.io/crates/v/zipcodes.svg?maxAge=2592000)](https://crates.io/crates/zipcodes)![Crates.io](https://img.shields.io/crates/d/zipcodes)

`Zipcodes` is a simple library for querying U.S. zipcodes. It pre-loads all zipcode data into memory at compile time, making it fast and efficient.

⚠️  The zipcode data was last updated on: **Feb. 16, 2025** ⚠️

## Installation

Add `zipcodes` to your project's dependencies:

```console
$ cargo add zipcodes
```

Or, add the following line to your `Cargo.toml`:

```toml
[dependencies]
zipcodes = "0.3.0"
```

## Quick Start

This example demonstrates the primary `matching` function and shows the full data structure for a located zipcode.

```rust
use zipcodes;

fn main() -> zipcodes::Result<()> {
    // Find zipcodes matching "77429"
    let results = zipcodes::matching("77429", None)?;

    // The `matching` function returns a `Vec`, as a 5-digit zipcode
    // isn't guaranteed to be unique across different localities.
    if let Some(zip) = results.first() {
        // Print the full debug output for the Zipcode struct
        println!("{:#?}", zip);
    }

    Ok(())
}
```

### Example Output

```text
Zipcode {
    acceptable_cities: [],
    active: true,
    area_codes: [
        "281",
        "832",
    ],
    city: "Cypress",
    country: "US",
    lat: "29.9857",
    long: "-95.6548",
    state: "TX",
    timezone: "America/Chicago",
    unacceptable_cities: [],
    world_region: "NA",
    zip_code: "77429",
    zip_code_type: "STANDARD",
}
```

## Examples

All fallible functions in this library return a `zipcodes::Result<T>`. The `?` operator is used for brevity in these examples.

### Validating a Zipcode

Use `is_real()` for a simple boolean check on a zipcode's existence.

```rust
use zipcodes::is_real;

fn main() -> zipcodes::Result<()> {
    // Returns Ok(true) for a real zipcode
    assert!(is_real("06903")?);

    // Returns Ok(false) for a non-existent zipcode
    assert!(!is_real("00000")?);

    Ok(())
}
```

### Advanced Filtering

The `filter_by()` function allows for powerful, custom queries using a vector of closures. This lets you find all zipcodes that match multiple specific criteria.

```rust
use zipcodes::{filter_by, Zipcode};

fn main() -> zipcodes::Result<()> {
    // Define filters to find all active "PO BOX" zipcodes in Massachusetts.
    // We use `Box<dyn Fn...>` to create a list of different closures.
    let filters: Vec<Box<dyn Fn(&Zipcode) -> bool>> = vec![
        Box::new(|z| z.state == "MA"),
        Box::new(|z| z.zip_code_type == "PO BOX"),
        Box::new(|z| z.active),
    ];

    let ma_po_boxes = filter_by(filters, None)?;

    println!("Found {} active PO Box zipcodes in Massachusetts.", ma_po_boxes.len());

    // Print the first 5 results
    for zip in ma_po_boxes.iter().take(5) {
        println!("- PO Box {} in {}", zip.zip_code, zip.city);
    }

    Ok(())
}
```

### Listing All Zipcodes

You can get a complete list of all zipcodes in the database.

```rust
use zipcodes::list_all;

let all_zips = list_all();
println!("There are {} zipcodes loaded in the database.", all_zips.len());
```

## Zipcode Data

The zipcode data is embedded directly into the library at compile time via a `build.rs` script. This ensures fast lookups at runtime without needing to read from a file or an external database.

## Contributing

Have an idea for a new feature? Feel free to open a pull request and contribute\!
