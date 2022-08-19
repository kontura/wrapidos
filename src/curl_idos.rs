use isahc::prelude::*;


pub async fn curl_idos(from: String, to: String, time: String) -> Result<String, isahc::Error> {
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
    let mut str_url = format!("https://idos.idnes.cz/vlakyautobusymhdvse/spojeni/vysledky/?f={}&t={}",
                              encoded_from,
                              encoded_to);
    if !time.is_empty() {
        str_url.push_str(&format!("&time={}", time));
    }
    str_url.push_str(&params);
    println!("{}", str_url);

    let mut response = isahc::get_async(&str_url).await?;
    Ok(response.text().await?)
}
