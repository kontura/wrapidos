use curl::easy::{Easy2, Handler, WriteError};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub fn curl_idos(mut from: String, mut to: String, time: String) -> String {
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();
    let mut params = String::new();
    if from.starts_with("Brno,") {
        params.push_str("&fc=302003");
        from = from.trim_start_matches("Brno,").to_string();
    }
    if to.starts_with("Brno,") {
        params.push_str("&tc=302003");
        to = to.trim_start_matches("Brno,").to_string();
    }
    let encoded_from = urlencoding::encode(&from);
    let encoded_to = urlencoding::encode(&to);
    let mut str_url = format!("https://idos.idnes.cz/vlakyautobusymhdvse/spojeni/vysledky/?f={}&t={}", encoded_from, encoded_to);
    if !time.is_empty() {
        str_url.push_str(&format!("&time={}", time));
    }
    str_url.push_str(&params);
    println!("{}", str_url);
    easy.url(&str_url).unwrap();
    easy.perform().unwrap();

    assert_eq!(easy.response_code().unwrap(), 200);
    let contents = easy.get_ref();
    String::from_utf8_lossy(&contents.0).to_string()
}
