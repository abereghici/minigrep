use colored::*;
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_insensitive: bool,
}

impl Config {
  pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
    // We're not interested in the first argument, because is the name of the program
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file name"),
    };

    let case_insensitive = match args.next() {
      Some(arg) => arg.eq(&String::from('I')),
      None => !env::var("CASE_INSENSITIVE").is_err(),
    };

    Ok(Config {
      query,
      filename,
      case_insensitive,
    })
  }
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let lower_query = query.to_lowercase();

  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&lower_query))
    .collect()
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(&query))
    .collect()
}

fn print_found_line(line: &str, config: &Config) {
  let query = &config.query;
  let mut line_to_print = line.replace(query, &query.red().to_string());

  if config.case_insensitive {
    let query = &config.query.to_lowercase();
    line_to_print = line_to_print.replace(query, &query.red().to_string());
  }

  println!("{}", line_to_print);
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(&config.filename)?;

  let lines = if config.case_insensitive {
    search_case_insensitive(&config.query, &contents)
  } else {
    search(&config.query, &contents)
  };

  for line in lines {
    print_found_line(line, &config);
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "the";
    let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

    assert_eq!(
      vec![
        "Then there's a pair of us - don't tell!",
        "To tell your name the livelong day"
      ],
      search(query, contents)
    );
  }

  #[test]
  fn case_insensitive() {
    let query = "THE";
    let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

    assert_eq!(
      vec![
        "Then there's a pair of us - don't tell!",
        "They'd banish us, you know.",
        "To tell your name the livelong day"
      ],
      search_case_insensitive(query, contents)
    );
  }
}
