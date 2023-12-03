use std::env;

/// Structure that holds the arguments passed to executable
///
/// Categorizes arguments like so:
/// * `executable`: path of called executable, this is typically the first argument received
/// * `commands`: list of commands passed to the executable
/// * `flags`: list of flags passed to the executable
/// * `paths`: list of paths passed after commands and flags
///
/// # Examples
/// A executable call can look like this
///
///         ./bin [-fx] [--verbose] <foo> <bar> -- <from> [to]
///
/// Which then would be parsed into
///
///         ArgumentParser {
///             executable: "./bin",
///             commands: ["foo, "bar"],
///             flags: ["f", "x", "verbose"],
///             paths: ["from", "to"]
///         }
#[derive(Debug)]
pub struct ArgumentConfig {
    pub executable: String,
    pub commands: Vec<String>,
    pub flags: Vec<String>,
    pub paths: Vec<String>,
}

impl ArgumentConfig {
    /// Creates a new argument config with empty values
    fn new() -> ArgumentConfig {
        ArgumentConfig {
            executable: String::new(),
            commands: vec![],
            flags: vec![],
            paths: vec![],
        }
    }

    /// Initializes the config reading the arguments passed to the executable
    ///
    /// # Returns
    ///
    /// Gives back ownership of an ArgumentConfig instance with all parameters loaded into itself
    pub fn init() -> ArgumentConfig {
        let mut args = env::args();
        let mut clean_args = vec![];

        loop {
            let arg = args.next();
            match arg {
                None => break,
                Some(arg) => clean_args.push(arg),
            }
        }

        let mut this = ArgumentConfig::new();
        this.parse_args(&mut clean_args);
        this
    }

    /// Reads arguments passed from the commandline into itself.
    fn parse_args(&mut self, args: &mut Vec<String>) {
        let commands = &mut self.commands;
        let flags = &mut self.flags;
        let paths = &mut self.paths;

        let mut path_divider = false;
        let mut iter = args.iter();
        self.executable = match iter.next() {
            None => String::new(),
            Some(arg) => arg.to_string(),
        };

        loop {
            let arg = match iter.next() {
                None => break,
                Some(arg) => arg,
            };

            if arg.eq(&"--") {
                path_divider = true;
                continue;
            }

            // We know that after '--' every arg is a filesystem path
            if path_divider {
                paths.push(arg.to_string());
                continue;
            }

            // Check for large flags
            if arg.starts_with("--") {
                flags.push(String::from(&arg[2..]));
                continue;
            }

            // Check for small flags
            if arg.starts_with("-") {
                // Grouped flags? eg "-fx"
                if arg.len() > 2 {
                    arg[1..].chars().for_each(|c| flags.push(String::from(c)));
                } else {
                    flags.push(String::from(&arg[1..]));
                }

                continue;
            }

            // If neither flags nor paths, then commands
            commands.push(arg.to_string());
        }
    }
}
