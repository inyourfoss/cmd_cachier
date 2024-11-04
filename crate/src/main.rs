use std::env;
use std::process::Command;

// Needed for stdin, stdout functionality
use std::process::Stdio;
use std::io::Write;
use std::io::Read;

use colored::*;

const REDIS_READ_WRITE_LATENCY_IN_MS: std::time::Duration = std::time::Duration::from_millis(20); // needs to be adjusted on weak hardware
                                                                                                 

// Solution found: https://stackoverflow.com/questions/27840394/how-can-a-rust-program-access-metadata-from-its-cargo-package
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION"); 


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
        "macos" => format!("{}/Library/Application Support",
                           std::env::var("HOME").expect("HOMEDIR Is not set.")
                          ),
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

    // Define the string to pass as stdin
    let config_string = format!( r#"
        port 0
        daemonize yes
        unixsocket '{}'
        unixsocketperm 700
        save ""
        appendonly no
        "#, socket);

    // Rest of function mostly generated by LLM. Seems to work well but will refactor later.

    let mut cmd = Command::new("redis-server")
        .arg("-")
        .stdin(Stdio::piped()) 
        .stdout(Stdio::piped()) 
        .spawn()
        .expect("Failed to start the server.");

    // Get a handle to stdin of the child process
    

    let stdin = cmd.stdin.as_mut().expect("Failed to open stdin");


    // Write the config string to the child process's stdin
    stdin.write_all(config_string.as_bytes()).expect("Failed to write to stdin");


    // Wait for the child process to finish
    let status = cmd.wait().expect("Failed to wait for command");

    // Write to stdout_buffer
    let mut stdout_buffer = Vec::new();
    let stdout = cmd.stdout.as_mut().expect("Failed to open stdout");
    stdout.read_to_end(&mut stdout_buffer).expect("Failed to read stdout");


    eprintln!("{}", String::from_utf8_lossy(&stdout_buffer));

    // Check if the command was successful
    if status.success() {
        eprintln!("Started server successfully.");
    } else {
        eprintln!("Server failed with: {:?}", status.code());
    }

    while ! (server_is_running()){
        eprint!("\rWaiting to connect to server...");
        std::thread::sleep(REDIS_READ_WRITE_LATENCY_IN_MS);
    }
    eprintln!();

    true
}

fn server_is_running() -> bool {

    let dbg_socket :String = socket();

    let error_message :String = format!("Connection string might be wrong.{dbg_socket}");

    let client = redis::Client::open(socket()).expect(&error_message);


    client.get_connection().is_ok() as bool
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    if ! server_is_running() {
        eprintln!("Server is not running yet. Starting server...");
        start_server(); 
    }

    let version = VERSION.unwrap_or("unknown");

    match sub_cmd().as_str() {
        "save" => save_cmd(cli_cmd(true))?,
        "query" => query_cmd(cli_cmd(true))?,
        "version" => println!("cmd_cachier version {version}"),
        "info" => redis_info()?,
        "meminfo" => redis_meminfo()?,
        "help" => display_help_page()?,
        "none" => display_help_page()?,
        _ => save_or_query_cmd(cli_cmd(false))?,
    }

    Ok(())
}

fn query_cmd(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

    eprintln!("Querying...");
    let client = redis::Client::open(socket())?;
    let mut _con = client.get_connection().expect("Could not establish connection. When querying");


    let joined_args:String = args.join(" ");

    let result: String = match redis::cmd("HGET").arg("cmd").arg(&joined_args).query(&mut _con) {
        Ok(a) => a,
        _ => format!("{}\n  {}\n{}\n", "KEY:".red().underline().bold(), &joined_args.bold(), "Not found in cache.".red().bold() )
    };


    print!("{}", result);

    Ok(())
}

fn save_or_query_cmd(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

    let client = redis::Client::open(socket()).expect("Error: Socket not found");
    let mut _con = client.get_connection().expect("Could not establish connection. When checking key.");

    let joined_args:String = args.join(" ");

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

    //TODO: Implement while loop key is not in cache.
    std::thread::sleep(REDIS_READ_WRITE_LATENCY_IN_MS);

    query_cmd(args)?;

    Ok(())
}


fn redis_info() -> Result<(), Box<dyn std::error::Error>> {
    let client = redis::Client::open(socket())?;
    let mut _con = client.get_connection()?;


    let response: String = redis::cmd("INFO").arg("MEMORY").query(&mut _con)?;
    let result_list: Vec<&str> = response.split('\n').collect();

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
    let used_mem_size :f64 = redis::cmd("MEMORY").arg("USAGE").arg("cmd").query(&mut _con)?;

    let to_megabytes = |i: f64| -> String { format!("{:.3}KiB", i / (1024.0)) } ;
    let to_kibibytes = |i: f64| -> String { format!("{:.3}MiB", i / (1024.0 * 1024.0)) } ;
    let kib_mib_cutoff_in_bytes: f64 = 1000.0 * 1024.0;
    

    let formated_mem_usage: String = 
        if used_mem_size > kib_mib_cutoff_in_bytes {
            to_kibibytes(used_mem_size)
        } else {
            to_megabytes(used_mem_size)
        };

     println!("MEMORY USED:\n{}", formated_mem_usage);

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

