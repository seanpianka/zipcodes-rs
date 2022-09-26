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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid format, zipcode must be of the format: \"#####\" or \"#####-####\"")]
    InvalidFormat,
    #[error("Invalid characters, zipcode may only contain digits and \"-\".")]
    InvalidCharacters,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn matching(zipcode: &str, zipcodes: Option<Vec<Zipcode>>) -> Result<Vec<Zipcode>> {
    let zipcode = clean_zipcode(zipcode)?;
    let zipcodes = zipcodes.as_ref().unwrap_or(&ZIPCODES);
    let matching_zipcodes = zipcodes.iter().filter(|z| z.zip_code == zipcode).cloned().collect::<Vec<_>>();
    debug_println!("is_real matched {:?} zipcodes", matching_zipcodes.len());
    Ok(matching_zipcodes)
}

pub fn is_real(zipcode: &str) -> Result<bool> {
    let zipcode = clean_zipcode(zipcode)?;
    let matching_zipcodes = matching(zipcode, None)?;
    Ok(!matching_zipcodes.is_empty())
}

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

pub fn list_all() -> Vec<Zipcode> {
    ZIPCODES.clone()
}

fn clean_zipcode(zipcode: &str) -> Result<&str> {
    let split_zipcode = zipcode.split("-").collect::<Vec<_>>();
    let zipcode = split_zipcode.first().ok_or(Error::InvalidFormat)?;
    if zipcode.len() != ZIPCODE_LENGTH {
        return Err(Error::InvalidFormat);
    }
    if !zipcode.chars().all(|c| c.is_numeric()) {
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
        match is_real("06902") {
            Ok(o) => {
                assert!(o)
            }
            Err(e) => {
                panic!("{}", e)
            }
        }
    }

    // TODO: Migrate remaining unittests for the python library.
}
