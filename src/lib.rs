//! Rust type bindings for the [Navan](https://navan.com) (formerly TripActions) Bookings API.
//!
//! This crate provides strongly-typed deserialization structs for the Navan `/v1/bookings` API
//! response. All fields are optional (except where the API guarantees presence) to handle
//! partial responses gracefully.
//!
//! # Example
//!
//! ```
//! use navan::Booking;
//!
//! let json = r#"{
//!   "uuid": "abc-123",
//!   "bookingType": "FLIGHT",
//!   "vendor": "American Airlines",
//!   "currency": "USD",
//!   "grandTotal": 342.50
//! }"#;
//!
//! let booking: Booking = serde_json::from_str(json).unwrap();
//! assert_eq!(booking.uuid, "abc-123");
//! assert_eq!(booking.vendor.as_deref(), Some("American Airlines"));
//! ```

use serde::{Deserialize, Serialize};

/// A booking record from the Navan Bookings API.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Booking {
  pub uuid: String,
  #[serde(default)]
  pub created: Option<String>,
  #[serde(default)]
  pub last_modified: Option<String>,
  #[serde(default)]
  pub pcc: Option<String>,
  #[serde(default)]
  pub booking_type: Option<String>,
  #[serde(default)]
  pub flight: Option<String>,
  #[serde(default)]
  pub route_type: Option<String>,
  #[serde(default)]
  pub airline_route: Option<String>,
  #[serde(default)]
  pub booking_id: Option<String>,
  #[serde(default)]
  pub vendor: Option<String>,
  #[serde(default)]
  pub preferred_vendor: Option<String>,
  #[serde(default)]
  pub corporate_discount_used: Option<String>,
  #[serde(default)]
  pub cabin: Option<String>,
  #[serde(default)]
  pub cabin_purchased: Option<String>,
  #[serde(default)]
  pub flown_cabin_class: Option<String>,
  #[serde(default)]
  pub booking_status: Option<String>,
  #[serde(default)]
  pub cancelled_at: Option<String>,
  #[serde(default)]
  pub cancellation_reason: Option<String>,
  #[serde(default)]
  pub lead_time_in_days: Option<i64>,
  #[serde(default)]
  pub start_date: Option<String>,
  #[serde(default)]
  pub end_date: Option<String>,
  #[serde(default)]
  pub booking_duration: Option<i64>,
  #[serde(default)]
  pub passengers: Vec<Passenger>,
  #[serde(default)]
  pub number_of_passengers: Option<i64>,
  #[serde(default)]
  pub segments: Vec<Segment>,
  #[serde(default)]
  pub booker: Option<Person>,
  #[serde(default)]
  pub trip_uuids: Vec<String>,
  #[serde(default)]
  pub departments: Vec<String>,
  #[serde(default)]
  pub cost_centers: Vec<String>,
  #[serde(default)]
  pub regions: Vec<String>,
  #[serde(default)]
  pub subsidiaries: Vec<String>,
  #[serde(default)]
  pub billable_entities: Vec<String>,
  #[serde(default)]
  pub currency: Option<String>,
  #[serde(default)]
  pub currency_exhange_rate_from_usd: Option<f64>,
  #[serde(default)]
  pub optimal_price: Option<f64>,
  #[serde(default)]
  pub payment_schedule: Option<String>,
  #[serde(default)]
  pub base_price: Option<f64>,
  #[serde(default)]
  pub unitary_price: Option<f64>,
  #[serde(default)]
  pub saving: Option<f64>,
  #[serde(default)]
  pub saving_missed: Option<f64>,
  #[serde(default)]
  pub tax: Option<f64>,
  #[serde(default)]
  pub resort_fee: Option<f64>,
  #[serde(default)]
  pub trip_fee: Option<f64>,
  #[serde(default)]
  pub handling_fees: Option<f64>,
  #[serde(default)]
  pub transaction_fees: Option<f64>,
  #[serde(default)]
  pub invoice_collection_fees: Option<f64>,
  #[serde(default)]
  pub booking_fee: Option<f64>,
  #[serde(default)]
  pub vip_fee: Option<f64>,
  #[serde(default)]
  pub navan_pro: Option<bool>,
  #[serde(default)]
  pub seats_fee: Option<f64>,
  #[serde(default)]
  pub extras_fees: Option<f64>,
  #[serde(default)]
  pub airline_credit_card_surcharge: Option<f64>,
  #[serde(default)]
  pub grand_total: Option<f64>,
  #[serde(default)]
  pub usd_grand_total: Option<f64>,
  #[serde(default)]
  pub vat: Option<f64>,
  #[serde(default)]
  pub exchange_amount: Option<f64>,
  #[serde(default)]
  pub exchange_fee: Option<f64>,
  #[serde(default)]
  pub net_charge: Option<f64>,
  #[serde(default)]
  pub gst: Option<f64>,
  #[serde(default)]
  pub hst: Option<f64>,
  #[serde(default)]
  pub qst: Option<f64>,
  #[serde(default)]
  pub travel_spend: Option<f64>,
  #[serde(default)]
  pub payment_method: Option<String>,
  #[serde(default)]
  pub name_on_credit_card: Option<String>,
  #[serde(default)]
  pub payment_method_used: Option<String>,
  #[serde(default)]
  pub payment_credit_card_type_name: Option<String>,
  #[serde(default)]
  pub company_payment_method: Option<String>,
  #[serde(default)]
  pub statement_description: Option<String>,
  #[serde(default)]
  pub expensed: Option<bool>,
  #[serde(default)]
  pub booking_method: Option<String>,
  #[serde(default)]
  pub policy_level: Option<String>,
  #[serde(default)]
  pub out_of_policy: Option<bool>,
  #[serde(default)]
  pub out_of_policy_description: Option<String>,
  #[serde(default)]
  pub out_of_policy_violations: Option<String>,
  #[serde(default)]
  pub out_of_policy_violation_types: Option<Vec<String>>,
  #[serde(default)]
  pub max_price_policy: Option<f64>,
  #[serde(default)]
  pub price_benchmark: Option<f64>,
  #[serde(default)]
  pub trip_bucks_earned: Option<f64>,
  #[serde(default)]
  pub trip_bucks_earned_usd: Option<f64>,
  #[serde(default)]
  pub origin: Option<Location>,
  #[serde(default)]
  pub destination: Option<Location>,
  #[serde(default)]
  pub trip_length: Option<String>,
  #[serde(default)]
  pub trip_description: Option<String>,
  #[serde(default)]
  pub approver_reason: Option<String>,
  #[serde(default)]
  pub approver_email: Option<String>,
  #[serde(default)]
  pub approval_changed_at: Option<String>,
  #[serde(default)]
  pub etickets: Vec<String>,
  #[serde(default)]
  pub invoice: Option<String>,
  #[serde(default)]
  pub invoice_number: Option<String>,
  #[serde(default)]
  pub pdf: Option<String>,
  #[serde(default)]
  pub domestic: Option<String>,
  #[serde(default)]
  pub inventory: Option<String>,
  #[serde(default)]
  pub flight_miles: Option<f64>,
  #[serde(default)]
  pub train_miles: Option<f64>,
  #[serde(default)]
  pub carbon_emissions: Option<f64>,
  #[serde(default)]
  pub carbon_offset_cost: Option<f64>,
  #[serde(default)]
  pub fare_class: Option<String>,
  #[serde(default)]
  pub fare_basis_code: Option<String>,
  #[serde(default)]
  pub purpose: Option<String>,
  #[serde(default)]
  pub company_office: Option<String>,
  #[serde(default)]
  pub trip_name: Option<String>,
  #[serde(default)]
  pub projects: Option<serde_json::Value>,
  #[serde(default)]
  pub bill_to_client: Option<String>,
  #[serde(default)]
  pub reason: Option<String>,
  #[serde(default)]
  pub cnr: Option<Cnr>,
  #[serde(default)]
  pub confirmation_number: Option<String>,
  #[serde(default)]
  pub seats: Vec<String>,
  #[serde(default)]
  pub credit: Option<Credit>,
  #[serde(default)]
  pub reshopping: Option<Reshopping>,
  #[serde(default)]
  pub hotel_code: Option<String>,
  #[serde(default)]
  pub hotel_chain: Option<String>,
  #[serde(default)]
  pub hotel_latitude: Option<f64>,
  #[serde(default)]
  pub hotel_longitude: Option<f64>,
  #[serde(default)]
  pub approval_status: Option<String>,
  #[serde(default)]
  pub car_type: Option<String>,
  #[serde(default)]
  pub event_name: Option<String>,
  #[serde(default)]
  pub invite_connection_method: Option<String>,
  #[serde(default)]
  pub invitation_type: Option<String>,
  #[serde(default)]
  pub logged_as_user_name: Option<String>,
  #[serde(default)]
  pub ppb_points_burned: Option<f64>,
  #[serde(default)]
  pub travel_agent_request_fee: Option<f64>,
  #[serde(default)]
  pub inventory_source: Option<String>,
  #[serde(default)]
  pub max_cancellation_loss: Option<f64>,
  #[serde(default)]
  pub custom_fields: Vec<CustomField>,
  #[serde(default)]
  pub transactions: Vec<Transaction>,
  #[serde(default)]
  pub gsa_rate: Option<f64>,
}

/// A flight/hotel/car segment within a booking.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
  #[serde(default)]
  pub start_timestamp: Option<i64>,
  #[serde(default)]
  pub end_timestamp: Option<i64>,
  #[serde(default)]
  pub start_local_date_time: Option<String>,
  #[serde(default)]
  pub end_local_date_time: Option<String>,
  #[serde(default)]
  pub departure: Option<Location>,
  #[serde(default)]
  pub arrival: Option<Location>,
  #[serde(default)]
  pub provider_code: Option<String>,
  #[serde(default)]
  pub provider_name: Option<String>,
  #[serde(default)]
  pub flight_number: Option<String>,
  #[serde(default)]
  pub airline_alliance: Option<String>,
  #[serde(default)]
  pub fare_class: Option<String>,
  #[serde(default)]
  pub cabin: Option<String>,
  #[serde(default)]
  pub cabin_purchased: Option<String>,
  #[serde(default)]
  pub hotel_chain: Option<String>,
  #[serde(default)]
  pub hotel_super_chain: Option<String>,
}

/// A geographic location (airport, hotel, city, etc.).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
  #[serde(default)]
  pub country: Option<String>,
  #[serde(default)]
  pub state: Option<String>,
  #[serde(default)]
  pub city: Option<String>,
  #[serde(default)]
  pub address: Option<String>,
  #[serde(default)]
  pub postal_code: Option<String>,
  #[serde(default)]
  pub airport_code: Option<String>,
}

/// A passenger on a booking.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Passenger {
  #[serde(default)]
  pub traveler_type: Option<String>,
  #[serde(default)]
  pub status: Option<String>,
  #[serde(default)]
  pub person: Option<Person>,
}

/// A person (traveler, booker, or manager).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
  #[serde(default)]
  pub uuid: Option<String>,
  #[serde(default)]
  pub name: Option<String>,
  #[serde(default)]
  pub email: Option<String>,
  #[serde(default)]
  pub employeed_id: Option<String>,
  #[serde(default)]
  pub manager_uuid: Option<String>,
  #[serde(default)]
  pub manager_name: Option<String>,
  #[serde(default)]
  pub manager_email: Option<String>,
  #[serde(default)]
  pub department: Option<String>,
  #[serde(default)]
  pub cost_center: Option<String>,
  #[serde(default)]
  pub region: Option<String>,
  #[serde(default)]
  pub subsidiary: Option<String>,
  #[serde(default)]
  pub billable_entity: Option<String>,
  #[serde(default)]
  pub passport: Option<Passport>,
}

/// Passport details for a traveler.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Passport {
  #[serde(default)]
  pub country_of_citizenship: Option<String>,
  #[serde(default)]
  pub country_of_issue: Option<String>,
  #[serde(default)]
  pub expires_on: Option<String>,
  #[serde(default)]
  pub issued_on: Option<String>,
}

/// A financial transaction associated with a booking.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
  #[serde(default)]
  pub total_price: Option<f64>,
  #[serde(default)]
  pub base_price: Option<f64>,
  #[serde(default)]
  pub tax: Option<f64>,
  #[serde(default)]
  pub payment_method: Option<String>,
  #[serde(default)]
  pub date_created: Option<String>,
  #[serde(default)]
  pub ticket_number: Option<String>,
  #[serde(default)]
  pub ticket_type: Option<String>,
  #[serde(default)]
  pub transaction_type: Option<String>,
}

/// Corporate negotiated rate (CNR) details.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cnr {
  #[serde(default)]
  pub published_price: Option<f64>,
  #[serde(default)]
  pub cnr_codes: Option<Vec<Option<String>>>,
  #[serde(default)]
  pub cnr_sources: Option<Vec<Option<String>>>,
  #[serde(default)]
  pub cnr_providers: Option<Vec<Option<String>>>,
}

/// Airline credit information.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Credit {
  #[serde(default)]
  pub credit_available: Option<f64>,
  #[serde(default)]
  pub credit_available_usd: Option<f64>,
  #[serde(default)]
  pub credit_used: Option<f64>,
  #[serde(default)]
  pub credit_used_usd: Option<f64>,
  #[serde(default)]
  pub credit_from_original_booking: Option<f64>,
  #[serde(default)]
  pub residual_credit: Option<f64>,
  #[serde(default)]
  pub credit_exchange_fee_usd: Option<f64>,
}

/// Reshopping (rebooking) details.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reshopping {
  #[serde(default)]
  pub reshopping_is_rebooked: Option<bool>,
  #[serde(default)]
  pub reshopping_new_price: Option<f64>,
  #[serde(default)]
  pub reshopping_original_price: Option<f64>,
  #[serde(default)]
  pub reshopping_new_ticket_number: Option<String>,
  #[serde(default)]
  pub reshopping_original_ticket_number: Option<String>,
}

/// A custom field key-value pair.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomField {
  #[serde(default)]
  pub name: Option<String>,
  #[serde(default)]
  pub value: Option<String>,
}

/// Paginated response wrapper from the Navan Bookings API.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingsResponse {
  pub data: Vec<Booking>,
  #[serde(default)]
  pub page: Option<Page>,
  #[serde(default)]
  pub next_cursor: Option<String>,
}

/// Pagination metadata.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
  #[serde(default)]
  pub next_cursor: Option<String>,
  #[serde(default)]
  pub total_pages: Option<i64>,
  #[serde(default)]
  pub current_page: Option<i64>,
  #[serde(default)]
  pub page_size: Option<i64>,
  #[serde(default)]
  pub total_elements: Option<i64>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize_minimal_booking() {
    let json = r#"{"uuid":"abc-123"}"#;
    let booking: Booking = serde_json::from_str(json).unwrap();
    assert_eq!(booking.uuid, "abc-123");
    assert!(booking.segments.is_empty());
    assert!(booking.passengers.is_empty());
  }

  #[test]
  fn deserialize_full_flight_booking() {
    let json = r#"{
      "uuid": "booking-1",
      "bookingType": "FLIGHT",
      "bookingStatus": "CONFIRMED",
      "vendor": "American Airlines",
      "currency": "USD",
      "basePrice": 72.56,
      "tax": 20.84,
      "grandTotal": 93.40,
      "created": "2026-03-18T20:47:07.743197Z",
      "confirmationNumber": "SPWFUL",
      "paymentMethod": "MASTERCARD 9008",
      "paymentCreditCardTypeName": "MasterCard",
      "fareClass": "B",
      "segments": [{
        "startTimestamp": 1778690880000,
        "endTimestamp": 1778696220000,
        "departure": {"airportCode": "LGA", "city": "New York", "state": "NY", "country": "US"},
        "arrival": {"airportCode": "BOS", "city": "Boston", "state": "MA", "country": "US"},
        "providerCode": "AA",
        "flightNumber": "4569",
        "fareClass": "B"
      }],
      "passengers": [{"person": {"name": "Jane Doe", "email": "jane@example.com"}}],
      "booker": {"name": "Jane Doe", "email": "jane@example.com", "billableEntity": "Acme Corp"},
      "etickets": ["00174099744980"],
      "transactions": [{
        "ticketType": "TICKET",
        "totalPrice": 93.40,
        "basePrice": 72.56,
        "tax": 20.84,
        "ticketNumber": "00174099744980",
        "dateCreated": "2026-03-18T20:47:10.427Z",
        "paymentMethod": "MASTERCARD 9008"
      }]
    }"#;

    let booking: Booking = serde_json::from_str(json).unwrap();
    assert_eq!(booking.booking_type.as_deref(), Some("FLIGHT"));
    assert_eq!(booking.vendor.as_deref(), Some("American Airlines"));
    assert_eq!(booking.grand_total, Some(93.40));
    assert_eq!(booking.segments.len(), 1);
    assert_eq!(
      booking.segments[0].departure.as_ref().unwrap().airport_code.as_deref(),
      Some("LGA")
    );
    assert_eq!(booking.transactions.len(), 1);
    assert_eq!(booking.transactions[0].ticket_number.as_deref(), Some("00174099744980"));
    assert_eq!(booking.booker.as_ref().unwrap().billable_entity.as_deref(), Some("Acme Corp"));
  }

  #[test]
  fn deserialize_hotel_booking() {
    let json = r#"{
      "uuid": "hotel-1",
      "bookingType": "HOTEL",
      "vendor": "Hilton Chicago",
      "startDate": "2026-08-03",
      "endDate": "2026-08-05",
      "hotelLatitude": 41.8721,
      "hotelLongitude": -87.6246,
      "hotelChain": "HH",
      "hotelCode": "CHIDTHH",
      "segments": [{
        "departure": {"city": "Chicago", "state": "IL", "country": "US"},
        "hotelChain": "HH",
        "hotelSuperChain": "Hilton"
      }]
    }"#;

    let booking: Booking = serde_json::from_str(json).unwrap();
    assert_eq!(booking.hotel_chain.as_deref(), Some("HH"));
    assert_eq!(booking.hotel_latitude, Some(41.8721));
    assert_eq!(booking.segments[0].hotel_super_chain.as_deref(), Some("Hilton"));
  }

  #[test]
  fn deserialize_bookings_response() {
    let json = r#"{
      "data": [{"uuid": "b1"}, {"uuid": "b2"}],
      "page": {"totalPages": 5, "currentPage": 1, "pageSize": 100, "totalElements": 450},
      "nextCursor": "cursor-abc"
    }"#;

    let resp: BookingsResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.data.len(), 2);
    assert_eq!(resp.next_cursor.as_deref(), Some("cursor-abc"));
    assert_eq!(resp.page.as_ref().unwrap().total_pages, Some(5));
  }

  #[test]
  fn roundtrip_serialize_deserialize() {
    let json = r#"{"uuid":"rt-1","bookingType":"CAR","vendor":"Hertz","carType":"COMPACT"}"#;
    let booking: Booking = serde_json::from_str(json).unwrap();
    let serialized = serde_json::to_string(&booking).unwrap();
    let roundtripped: Booking = serde_json::from_str(&serialized).unwrap();
    assert_eq!(roundtripped.car_type.as_deref(), Some("COMPACT"));
  }

  #[test]
  fn unknown_fields_are_ignored() {
    let json = r#"{"uuid":"uf-1","someNewField":"value","anotherUnknown":42}"#;
    let booking: Booking = serde_json::from_str(json).unwrap();
    assert_eq!(booking.uuid, "uf-1");
  }
}
