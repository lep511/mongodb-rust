use serde::{ Deserialize, Serialize };
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Host {
    pub host_id: Option<String>,
    pub host_url: Option<String>,
    pub host_name: Option<String>,
    pub host_location: Option<String>,
    pub host_about: Option<String>,
    pub host_response_time: Option<String>,
    pub host_thumbnail_url: Option<String>,
    pub host_picture_url: Option<String>,
    pub host_response_rate: Option<i32>,
    pub host_is_superhost: Option<bool>,
    pub host_has_profile_pic: Option<bool>,
    pub host_identity_verified: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub coordinates: Option<Vec<f64>>,
    pub is_location_exact: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    pub street: Option<String>,
    pub government_area: Option<String>,
    pub market: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub location: Option<Location>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Review {
    pub _id: Option<String>,
    pub date: Option<String>,
    pub listing_id: Option<String>,
    pub reviewer_id: Option<String>,
    pub reviewer_name: Option<String>,
    pub comments: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Listing {
    pub _id: Option<i32>,
    pub listing_url: Option<String>,
    pub name: Option<String>,
    pub summary: Option<String>,
    pub space: Option<String>,
    pub description: Option<String>,
    pub neighborhood_overview: Option<String>,
    pub notes: Option<String>,
    pub transit: Option<String>,
    pub access: Option<String>,
    pub interaction: Option<String>,
    pub house_rules: Option<String>,
    pub property_type: Option<String>,
    pub room_type: Option<String>,
    pub bed_type: Option<String>,
    pub minimum_nights: Option<i32>,
    pub maximum_nights: Option<i32>,
    pub cancellation_policy: Option<String>,
    pub last_scraped: Option<String>,
    pub calendar_last_scraped: Option<String>,
    pub first_review: Option<String>,
    pub last_review: Option<String>,
    pub accommodates: Option<i32>,
    pub bedrooms: Option<f64>,
    pub beds: Option<f64>,
    pub number_of_reviews: Option<i32>,
    pub bathrooms: Option<f64>,
    pub amenities: Option<Vec<String>>,
    pub price: Option<i32>,
    pub security_deposit: Option<f64>,
    pub cleaning_fee: Option<f64>,
    pub extra_people: Option<i32>,
    pub guests_included: Option<i32>,
    pub images: Option<Value>,
    pub host: Option<Host>,
    pub address: Option<Address>,
    pub availability: Option<Value>,
    pub review_scores: Option<Value>,
    pub reviews: Option<Vec<Review>>,
    pub text_embeddings: Option<Vec<f64>>,
}