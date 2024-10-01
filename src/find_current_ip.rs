use std::f32::consts::PI;

pub fn run() -> Result<String, reqwest::Error> {
    let mut address = String::new();
    let source = "https://whatismyip.akamai.com/";

    let response = reqwest::blocking::get(source)?;

    match response.error_for_status() {
        Ok(result) => return result.text(),
        Err(e) =>   return Err(e),
    };
}
