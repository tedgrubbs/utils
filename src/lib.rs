
// noticed that I was doing a lot of copy/pasting of this function in my log and log_server codes. So starting this as a central utilities 
// module.

pub mod utils {
  use std::{fs, io, collections::HashMap};

  pub fn read_file_into_hash(filepath: &str, allowed_opts: Option<&[&str]>, result_hash: &mut HashMap<String, String>) -> io::Result<()> {

    // will try to treat filepath as an actual file path, but if it doesn't exist will assume that the filepath string is 
    // actually the contents of the file.
    let options = match fs::read_to_string(filepath) {
      Ok(v) => v,
      Err(_e) => filepath.to_string()
    };

    for l in options.lines() {

      if l.len() == 0 || l.chars().nth(0).unwrap() == '#' {
        continue;
      }

      let (option, value) = l.split_once(":").unwrap();
      let option = option.trim();
      let value = value.trim();


      if allowed_opts != None && !allowed_opts.unwrap().contains(&option) {
        panic!("Unknown config parameter found: {}", option)
      }

      // need to handle case of multiple options in line
      result_hash.insert(
        option.to_string(),
        value.to_string()
      );

    }

    Ok(())
  }
}
