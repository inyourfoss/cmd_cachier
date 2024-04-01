use std::env;
use std::process::Command;
//use memcache;
use redis;

use colored::*;

//const MAX_CONNECTION_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(4000); 
const REDIS_READ_WRITE_LATENCY_IN_MS: std::time::Duration = std::time::Duration::from_millis(20); // needs to be adjusted on weak hardware

fn sub_cmd() -> String {
    match env::args().nth(1) {
        Some(a) => a,
        None => "none".to_string(),
    }
}

fn cli_cmd(has_subcommand: bool) -> Vec<String> {
    let mut offset: u8 = 1;

    if has_subcommand {
        offset += 1;
    }

    env::args().skip(offset.into()).collect()
}


fn socket_no_prefix() -> String {
    let os : &str = std::env::consts::OS;
    let runtime_dir : String = match os {
        "linux" => std::env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR is not set."),
        "macos" => "/tmp".to_string(),
        //"windows" => "/run/user/1000",
        _ => panic!("Could not determine platform.")
    };

    format!("{runtime_dir}/cmd_cachier_redis.sock")
}

fn socket() -> String {
    let socket = socket_no_prefix();
    format!("unix://{socket}")
}

fn start_server() -> bool {
    let socket = socket_no_prefix();

    let _execute_command = Command::new("sh")
        .arg("-c")
        .arg(format!("printf 'daemonize yes\nport 0\nunixsocket {socket}\nunixsocketperm 700\nsave \"\"\nappendonly no\n' | redis-server -").as_str())
        .output()
        .expect("Failed to start the server.");

    while ! (server_is_running()){
        println!("Waiting to connect to server...");
        std::thread::sleep(REDIS_READ_WRITE_LATENCY_IN_MS);
    }

    return true;
}

fn server_is_running() -> bool {

    let dbg_socket :String = socket();

    let client = redis::Client::open(socket()).expect(format!("Connection string might be wrong.{dbg_socket}").as_str());

    let result = match client.get_connection() {
        Ok(_) => true,
        Err(_) => false
    };

    return result;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    if ! server_is_running() {
        println!("Server is not running yet. Starting server...");
        start_server(); 
    }

    match sub_cmd().as_str() {
        "save" => save_cmd(cli_cmd(true))?,
        "query" => query_cmd(cli_cmd(true))?,
        "info" => redis_info()?,
        "meminfo" => redis_meminfo()?,
        "help" => display_help_page()?,
        "none" => display_help_page()?,
        _ => save_or_query_cmd(cli_cmd(false))?,
    }

    Ok(())
}

// Most performance critical function.
fn query_cmd(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

    //let client = memcache::connect(socket()).unwrap();
    //
    println!("Querying...");
    let client = redis::Client::open(socket())?;
    let mut _con = client.get_connection().expect("Could not establish connection.");

    //let dbg_socket = socket();
    //println!("{dbg_socket}");

    let joined_args:String = args.join(" ");

    //let result: String = client.get(&joined_args).unwrap().expect(format!("Key not found.").as_str());

    let result: String = match redis::cmd("HGET").arg("cmd").arg(&joined_args).query(&mut _con) {
        Ok(a) => a,
        _ => format!("{}\n  {}\n{}\n", "KEY:".red().underline().bold(), &joined_args.bold(), "Not found in cache.".red().bold() )
    };


    print!("{}", result);

    Ok(())
}

fn save_or_query_cmd(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    
    //let client = memcache::connect(socket()).unwrap();
    let client = redis::Client::open(socket()).expect("Error: Socket not found");
    let mut _con = client.get_connection().expect("Could not establish connection.");

    let joined_args:String = args.join(" ");

    //let redis_result: Option<String> = client.get(&joined_args).unwrap();
        let redis_result: Option<String> = redis::cmd("HGET").arg("cmd").arg(&joined_args).query(&mut _con)?;

    match redis_result {
        Some(a) => print!("{}", a),
        _ => save_cmd(args)?
    }

    Ok(())
}


fn save_cmd(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

    let client = redis::Client::open(socket())?;
    let mut _con = client.get_connection()?;
    //let client = memcache::connect(socket()).unwrap();

    let joined_args = args.join(" ");

    let execute_command = Command::new(&args[0]).args(&args[1..])
        .output().expect("Could not execute redis-cli.");

    let cmd_stdout: String = String::from_utf8_lossy(&execute_command.stdout).to_string();

    eprintln!("{}{} writing {}{}{} to cache.", 
              "INFO".yellow().bold().underline(), ":".yellow().bold(), 
              "\"".yellow().bold(),
              &joined_args.yellow().bold(),
              "\"".yellow().bold(),
              );

    redis::cmd("HSET").arg("cmd").arg(joined_args).arg(cmd_stdout).execute(&mut _con);
    //let _ = client.set(&joined_args, cmd_stdout, 0);
    

    std::thread::sleep(REDIS_READ_WRITE_LATENCY_IN_MS);

    query_cmd(args)?;

    Ok(())
}


fn redis_info() -> Result<(), Box<dyn std::error::Error>> {
    let client = redis::Client::open(socket())?;
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
    let client = redis::Client::open(socket())?;
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

