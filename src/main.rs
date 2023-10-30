use std::env;
use std::process::Command;

use colored::*;

const REDIS_READ_WRITE_LATENCY_IN_MS: std::time::Duration = std::time::Duration::from_millis(20);

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let first_arg = match env::args().nth(1) {
        Some(a) => a,
        None => "default".to_string(),
    };

    match first_arg.as_str() {
        "save" => save_cmd(env::args().skip(2).collect())?,
        "query" => query_cmd(env::args().skip(2).collect())?,
        "info" => redis_info()?,
        "meminfo" => redis_meminfo()?,
        "help" => display_help_page()?,
        _ => save_or_query_cmd(env::args().skip(1).collect())?,
    }

    Ok(())
}

fn query_cmd(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut _con = client.get_connection()?;

    let joined_args = args.join(" ");

    let result: String = redis::cmd("HGET").arg("cmd").arg(joined_args).query(&mut _con)?;

    print!("{}", result);

    Ok(())
}


fn save_cmd(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut _con = client.get_connection()?;

    let joined_args = args.join(" ");

    let output = Command::new(&args[0]).args(&args[1..])
        .output().expect("Could not execute redis-cli.");

    let cmd_stdout: String = String::from_utf8_lossy(&output.stdout).to_string();

    eprintln!("{}{} writing {}{}{} to cache.", 
              "INFO".yellow().bold().underline(), ":".yellow().bold(), 
              "\"".yellow().bold(),
              &joined_args.yellow().bold(),
              "\"".yellow().bold(),
              );

    redis::cmd("HSET").arg("cmd").arg(joined_args).arg(cmd_stdout).execute(&mut _con);

    std::thread::sleep(REDIS_READ_WRITE_LATENCY_IN_MS);

    query_cmd(args)?;

    Ok(())
}

fn save_or_query_cmd(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

    match query_cmd(args.clone()) {
        Ok(()) => neet()?,
        _ => save_cmd(args.clone())?,
    };

    Ok(())
}

fn neet() -> Result<(), Box<dyn std::error::Error>> {
    //eprintln!("{}", "Success!".green().underline().bold());
    Ok(())
}

fn redis_info() -> Result<(), Box<dyn std::error::Error>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut _con = client.get_connection()?;

    
    let response: String = redis::cmd("INFO").arg("MEMORY").query(&mut _con)?;
    let result_list: Vec<&str> = response.split("\n").collect();
    
    println!("{}", "REDIS MEMORY INFO:".yellow().bold().underline());
    for result in result_list {
        if result.contains("human") {
            println!("{}", result);
        }
    }

    Ok(())
}

fn redis_meminfo() -> Result<(), Box<dyn std::error::Error>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut _con = client.get_connection()?;
    
    let response :f64 = redis::cmd("MEMORY").arg("USAGE").arg("cmd").query(&mut _con)?;
    let conversion_number: f64 = 1024.0;
    let mem_usage_kb: f64 = response/(conversion_number);
    let mem_usage_mb: f64 = response/(conversion_number*conversion_number);

    println!("Bytes:\t{:.1}", response);
    println!("KiB:\t{:.3}", mem_usage_kb);
    println!("MiB:\t{:.3}", mem_usage_mb);

    Ok(())
}

fn display_help_page() -> Result<(), Box<dyn std::error::Error>> {
//    eprintln!("{}", "Success!".green().underline().bold());
    let help_page = r#"cmd_cachier help page

USAGE
Basic usage (Saves automatically if command is not already in cache but otherwise prints from cache):
    $ cmd_cachier ANY_COMMAND

Use with subcommand:
    $ cmd_cachier SUBCOMMAND [ANY_COMMAND]

MORE HELP
View man page:
    $ man cmd_cachier

SUBCOMMANDS
Force cache refresh for a command:
    $ cmd_cachier save ANY_COMMAND

Force query for a command:
    $ cmd_cachier query ANY_COMMAND

Display memory usage:
    $ cmd_cachier meminfo

Display help page:
    $ cmd_cachier help
"#;
    println!("{}", help_page);
    Ok(())
}

