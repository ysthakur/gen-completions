use regex::Regex;

fn parse(s: &str) -> Option<()> {
  match s.find("\nOPTIONS") {
    Some(options_header) => {
      let re = Regex::new("").unwrap();
      todo!()
    }
    None => None
  }
}
