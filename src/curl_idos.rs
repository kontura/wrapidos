use curl::easy::{Easy2, Handler, WriteError};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub fn curl_idos(from: String, to: String, time: String) -> String {
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();
    let mut params = String::new();
    let from_low = from.to_ascii_lowercase();
    if from_low.starts_with("brno,") {
        params.push_str("&fc=8");
    }
    let to_low = to.to_ascii_lowercase();
    if to_low.starts_with("brno,") {
        params.push_str("&tc=8");
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
