pub fn run() -> Result<String, reqwest::Error> {
    let source = "https://whatismyip.akamai.com/";

    let response = reqwest::blocking::get(source)?;

    let address = match response.error_for_status() {
        Ok(result) => result.text(),
        Err(e) =>   return Err(e),
    };

    return address;
}
