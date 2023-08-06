
mod parse_man;


pub struct Parsed {
  cmd_name: String,
  opts: Vec<Opt>,
}

pub struct Opt {
  name: String,
  desc: String,
}
