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
        "save" => fleet(env::args().skip(2).collect())?,
        "query" => yeet(env::args().skip(2).collect())?,
        "info" => meet()?,
        "meminfo" => peet()?,
        _ => keet(env::args().skip(1).collect())?,
    }

    Ok(())
}

fn yeet(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut _con = client.get_connection()?;

    let joined_args = args.join(" ");

    let result: String = redis::cmd("HGET").arg("cmd").arg(joined_args).query(&mut _con)?;

    print!("{}", result);

    Ok(())
}


fn fleet(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

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

    yeet(args)?;

    Ok(())
}

fn keet(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

    match yeet(args.clone()) {
        Ok(()) => neet()?,
        _ => fleet(args.clone())?,
    };

    Ok(())
}

fn neet() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("{}", "Success!".green().underline().bold());
    Ok(())
}

fn meet() -> Result<(), Box<dyn std::error::Error>> {
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

fn peet() -> Result<(), Box<dyn std::error::Error>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut _con = client.get_connection()?;
    
    let response :f64 = redis::cmd("MEMORY").arg("USAGE").arg("cmd").query(&mut _con)?;
    let mem_usage_kb: f64 = response/1e3;
    let mem_usage_mb: f64 = response/1e6;

    println!("Bytes:\t\t{:.1}", response);
    println!("Kilobytes:\t{:.3}", mem_usage_kb);
    println!("Megabytes:\t{:.3}", mem_usage_mb);

    Ok(())
}
