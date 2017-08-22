extern crate reqwest;

use std::io::Read;

/// `shorten` returns a `String` representing the shortened URL
/// `varys` uses https://is.gd/ to shorten a url
///
/// # Arguments
///
/// * `url` - URL to shorten
///
/// # Panics
///
/// `shorten` panics if the request is not successful.
///
/// # Example
///
/// `let shortened_url: String = varys::shorten("https://www.rust-lang.org/");
///  // =>  https://is.gd/jK51hw
/// `
///
pub fn shorten(url: &'static str) -> String {
    let url = &format!("https://is.gd/create.php?format=simple&url={}", url)[..];
    let mut response = reqwest::get(url).unwrap();
    assert!(response.status().is_success());
    let mut body = String::new();
    response.read_to_string(&mut body);
    body
}