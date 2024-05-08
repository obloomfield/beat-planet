pub struct Cookies {
  pub csrf: Option<String>,
  pub uid: Option<String>,
}

pub fn get_cookies(cookie_string: String) -> Cookies {
  Cookies {
    csrf: wasm_cookies::cookies::get(&cookie_string, "csrftoken").map(|result| result.unwrap()),
    uid: wasm_cookies::cookies::get(&cookie_string,"uid").map(|result| result.unwrap()),
  }
}