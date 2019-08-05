/*
 * Parse the command to be aliased into a
 * main app name and the args given to
 * the app.
 * i.e app -> [git], args -> [status]
*/

// Use
use std::env;
use std::io;
use std::process;

// opts

// Parsed command
#[derive(Debug)]
struct ParsedCommand<'a> {
  command: &'a str,
  args: Option<Vec<&'a str>>,
}

// Alias entry
#[derive(Debug)]
struct AliasEntry<'a> {
  alias: &'a str,
  entry: ParsedCommand<'a>,
}

// New alias
impl<'a> AliasEntry<'a> {
  fn new(alias: &'a str, entry: ParsedCommand<'a>) -> Self {
    Self {
      alias: alias,
      entry: entry,
    }
  }
}

// New parsed command
impl<'a> ParsedCommand<'a> {
  fn new(entry: &'a str) -> Self {
    let entry_split: Vec<&str> = entry.split(" ").collect();
    let command = entry_split[0];

    if entry_split.len() > 1 {
      let args = &entry_split[1..];
      return Self {
        command: command,
        args: Some(args.to_owned()),
      };
    } else {
      return Self {
        command: command,
        args: None,
      };
    }
  }
}

// Parse
pub fn parse_args() -> Result<(), io::Error> {
  let args: Vec<String> = env::args().collect();
  let help = format!(
    " uhm [Alias] 
  
    This is a tool to alias long commands
    
    ##############################
    
    add: Your Alias [SPACE] Command
    
    list: Lists all aliases"
  );
  // if no args print help and get on with it
  if args.len() < 2 {
    println!("{}", help);
    process::exit(0);
  }

  // Add
  let args_without_path = &args[1..];
  if args_without_path[0] == "add" && args_without_path.len() == 3 {
    let alias = args_without_path[1].clone();
    let entry = args_without_path[2].clone();
    add(alias.as_str(), entry.as_str());
  }

  // list
  if args_without_path[0] == "list" {
    println!("Unimplemented!");
  }

  Ok(())
}

// Add an alias
fn add(alias: &str, entry: &str) {
  let parsed_result = ParsedCommand::new(entry);
  let alias_result = AliasEntry::new(alias, parsed_result);
  println!("{:?}", alias_result);
}
