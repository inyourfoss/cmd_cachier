pub fn display_help_page() 
    -> Result<(), Box<dyn std::error::Error>> {

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

Display memory usage:
    $ cmd_cachier meminfo

Display help page:
    $ cmd_cachier help
"#;
    println!("{}", help_page);
    Ok(())
}

