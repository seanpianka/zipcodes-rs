use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use bzip2::read::{BzDecoder};
use debug_print::debug_println;

const ZIPCODE_LENGTH: usize = 5;

static ZIPCODE_BYTES_BZIP: &'static [u8] = include_bytes!("zips.json.bz2");
lazy_static! {
    static ref ZIPCODES: Vec<Zipcode> = {
        let mut decompressor = BzDecoder::new(ZIPCODE_BYTES_BZIP);
        let mut zipcode_json_bytes = String::new();
        decompressor.read_to_string(&mut zipcode_json_bytes).unwrap();
        match serde_json::from_str::<Vec<Zipcode>>(zipcode_json_bytes.as_str()) {
            Ok(o) => o,
            Err(e) => { panic!("failed to deserialize zipcode database: {}", e); }
        }
    };
}

/// Describes different types of errors with supplied zipcodes during parsing.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid format, zipcode must be of the format: \"#####\" or \"#####-####\"")]
    InvalidFormat,
    #[error("Invalid characters, zipcode may only contain digits and \"-\".")]
    InvalidCharacters,
}

/// A result type where the error is an `Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// Determine whether a supplied zipcode matches any existing zipcode. The supplied zipcode must be of the format: "#####", "#####-####", or "##### ####".
pub fn matching(zipcode: &str, zipcodes: Option<Vec<Zipcode>>) -> Result<Vec<Zipcode>> {
    let zipcode = clean_zipcode(zipcode)?;
    let zipcodes = zipcodes.as_ref().unwrap_or(&ZIPCODES);
    let matching_zipcodes = zipcodes.iter().filter(|z| z.zip_code == zipcode).cloned().collect::<Vec<_>>();
    debug_println!("is_real matched {:?} zipcodes for {}", matching_zipcodes.len(), zipcode);
    Ok(matching_zipcodes)
}

/// Returns true if the supplied zipcode is a valid zipcode.
///
/// This is mainly a wrapper around `is_real` that returns a `Result` instead of a `bool`.
pub fn is_real(zipcode: &str) -> Result<bool> {
    let zipcode = clean_zipcode(zipcode)?;
    let matching_zipcodes = matching(zipcode, None)?;
    Ok(!matching_zipcodes.is_empty())
}

/// Using a supplied list of filt-er-functions, return a filtered list of zipcodes.
///
/// By default, the supplied list of zipcodes is everything stored in the
/// database. However, an optional list of override zipcodes can be supplied.
pub fn filter_by<F>(filters: Vec<F>, zipcodes: Option<Vec<Zipcode>>) -> Result<Vec<Zipcode>>
                    where F: Fn(&Zipcode) -> bool {
    let zipcodes = zipcodes.as_ref().unwrap_or(&ZIPCODES);
    Ok(zipcodes.iter().filter(|z| {
        for filter in &filters {
            if !filter(z) {
                return false;
            }
        }
        true
    }).cloned().collect::<Vec<_>>())
}

/// Retrieve a list of all zipcodes in the database.
pub fn list_all() -> Vec<Zipcode> {
    ZIPCODES.clone()
}

fn clean_zipcode(zipcode: &str) -> Result<&str> {
    let zipcode = zipcode.trim();
    if zipcode.len() < ZIPCODE_LENGTH {
        return Err(Error::InvalidFormat);
    }
    let split_zipcode = &zipcode[..ZIPCODE_LENGTH];
    if !split_zipcode.chars().all(|c| c.is_numeric()) {
        return Err(Error::InvalidCharacters);
    }
    Ok(zipcode)
}

/// The available fields in the zipcode database.
///
/// 'acceptable_cities': [],
/// 'active': True,
/// 'area_codes': ['281', '832'],
/// 'city': 'Cypress',
/// 'country': 'US',
/// 'county': 'Harris County',
/// 'lat': '29.9857',
/// 'long': '-95.6548',
/// 'state': 'TX',
/// 'timezone': 'America/Chicago',
/// 'unacceptable_cities': [],
/// 'world_region': 'NA',
/// 'zip_code': '77429',
/// 'zip_code_type': 'STANDARD'}[
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Zipcode {
    pub acceptable_cities: Vec<String>,
    pub active: bool,
    pub area_codes: Vec<String>,
    pub city: String,
    pub country: String,
    pub lat: String,
    pub long: String,
    pub state: String,
    pub timezone: String,
    pub unacceptable_cities: Vec<String>,
    pub world_region: String,
    pub zip_code: String,
    pub zip_code_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_real_zipcodes() {
        assert!(is_real("06903").unwrap())
    }

    #[test]
    fn should_return_no_zipcodes() {
        for zc in &[
            "00000",
            "00000-0000",
            "00000 0000",
        ] {
            assert!(matching(zc, None).unwrap().is_empty())
        }
    }

    #[test]
    fn should_fail_to_find_zipcodes_not_included_in_overrides() {
        let zc = "06903";
        matching(zc, None).unwrap();
        assert!(matching(zc, Some(matching("06904", None).unwrap())).unwrap().is_empty());
    }

    // TODO: Migrate remaining unittests for the python library.
}
