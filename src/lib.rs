
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
      let line: Vec<&str> = l.split_whitespace().collect();
      if line.len() == 0 || line[0].chars().nth(0).unwrap() == '#' {
        continue;
      }

      if allowed_opts != None && !allowed_opts.unwrap().contains(&line[0]) {
        panic!("Unknown config parameter found: {}", line[0])
      }

      // need to handle case of multiple options in line
      result_hash.insert(
        line[0].to_string(),
        line[1..].join(" ")
      );

    }

    Ok(())
  }
}
