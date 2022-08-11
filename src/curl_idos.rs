use curl::easy::{Easy2, Handler, WriteError};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub fn curl_idos(from: String, to: String) -> String {
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();
    let encoded_from = urlencoding::encode(&from);
    let encoded_to = urlencoding::encode(&to);
    let str_url = &format!("https://idos.idnes.cz/vlakyautobusymhdvse/spojeni/vysledky/?f={}&fc=8&t={}&tc=8", encoded_from, encoded_to);
    println!("{}", str_url);
    easy.url(&str_url).unwrap();
    easy.perform().unwrap();

    assert_eq!(easy.response_code().unwrap(), 200);
    let contents = easy.get_ref();
    String::from_utf8_lossy(&contents.0).to_string()
}
