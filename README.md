# navan

Rust type bindings for the [Navan](https://navan.com) (formerly TripActions) `/v1/bookings` API.

A data-only library — no HTTP client, no runtime logic — just `serde` structs that mirror the API's JSON shape.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
navan = "0.1"
```

Deserialize a booking from JSON:

```rust
use navan::Booking;

let json = r#"{
  "uuid": "abc-123",
  "bookingType": "FLIGHT",
  "vendor": "American Airlines",
  "currency": "USD",
  "grandTotal": 342.50
}"#;

let booking: Booking = serde_json::from_str(json).unwrap();
assert_eq!(booking.uuid, "abc-123");
assert_eq!(booking.vendor.as_deref(), Some("American Airlines"));
```

Parse a paginated API response:

```rust
use navan::BookingsResponse;

let resp: BookingsResponse = serde_json::from_str(&body).unwrap();
for booking in &resp.data {
    println!("{}: {:?}", booking.uuid, booking.booking_type);
}
if let Some(cursor) = &resp.next_cursor {
    // fetch next page
}
```

## Types

| Struct | Description |
|---|---|
| `Booking` | Top-level booking record (~100 optional fields covering flights, hotels, cars, and rail) |
| `BookingsResponse` | Paginated API response wrapper (`data` + `Page`) |
| `Segment` | Flight/hotel/car segment within a booking |
| `Location` | Geographic location (airport, hotel, city) |
| `Passenger` | A passenger on a booking |
| `Person` | A person (traveler, booker, or manager) |
| `Passport` | Passport details for a traveler |
| `Transaction` | Financial transaction associated with a booking |
| `Cnr` | Corporate negotiated rate details |
| `Credit` | Airline credit information |
| `Reshopping` | Rebooking details |
| `CustomField` | Custom field key-value pair |
| `Page` | Pagination metadata |

## Design

- All structs use `#[serde(rename_all = "camelCase")]` to map between Rust snake_case and the API's camelCase JSON
- Nearly every field is `Option<T>` or `Vec<T>` with `#[serde(default)]` to handle partial API responses
- Unknown JSON fields are silently ignored — deserialization is forward-compatible with API changes
- Dependencies are intentionally minimal: only `serde` and `serde_json`

## License

MIT
