# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Rust type bindings crate for the Navan (formerly TripActions) `/v1/bookings` API. A data-only library — no HTTP client, no runtime logic — just serde structs that mirror the API's JSON shape.

## Build & Test Commands

```bash
cargo build          # compile
cargo test           # run all tests
cargo test <name>    # run a single test by name (e.g. cargo test deserialize_hotel_booking)
cargo clippy         # lint
```

## Architecture

Single-file library (`src/lib.rs`) containing:

- **`Booking`** — top-level struct with ~100 optional fields covering flights, hotels, cars, and rail
- **`BookingsResponse`** — paginated API response wrapper (`data` + `Page` cursor/pagination)
- Supporting structs: `Segment`, `Location`, `Passenger`, `Person`, `Passport`, `Transaction`, `Cnr`, `Credit`, `Reshopping`, `CustomField`, `Page`

All structs use `#[serde(rename_all = "camelCase")]` to map between Rust snake_case and the API's camelCase JSON. Nearly every field is `Option<T>` or `Vec<T>` with `#[serde(default)]` to handle partial API responses. Unknown JSON fields are silently ignored (serde default behavior — no `deny_unknown_fields`).

## Conventions

- Dependencies are intentionally minimal: only `serde` and `serde_json`
- No `deny_unknown_fields` — the API may add fields at any time, and deserialization must remain forward-compatible
- Note: the API has a known typo `employeed_id` (maps to `employeedId` in JSON) — this matches the actual API response and should not be "fixed"
