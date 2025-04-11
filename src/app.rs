use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::config::ConfigEntry;

use super::config::Config;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

pub struct LunchrApp {
    config: Config,
}

impl LunchrApp {
    pub fn new(config: Config) -> Self {
        LunchrApp { config }
    }

    fn is_command_running(&self, pattern: &str) -> bool {
        let is_docker = pattern.contains("docker");
        let query = if is_docker {
            format!("docker ps | grep -E \"{}\"", pattern.replace("docker", ""))
        } else {
            format!("ps aux | grep -E \"{}\"", pattern)
        };
        let output = Command::new("sh")
            .arg("-c")
            .arg(&query)
            .output()
            .expect("Failed to execute health check command");
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let result_lines: Vec<&str> = stdout
                .lines()
                .filter(|line| !line.contains("grep"))
                .collect();
            return !result_lines.is_empty();
        }
        return false;
    }

    fn start_command(&self, command: &ConfigEntry) -> Result<(), Box<dyn std::error::Error>> {
        println!("Staring command {}...", command.name);
        Command::new("sh")
            .arg("-c")
            .arg(&command.start_command)
            .current_dir(&command.cwd)
            .stdout(Stdio::null())
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        thread::sleep(Duration::from_millis(250));
        Ok(())
    }

    fn stop_command(&self, command: &ConfigEntry) -> Result<(), Box<dyn std::error::Error>> {
        println!("Stopping command {}...", command.name);
        if command.kill_command.is_empty() {
            Command::new("pkill")
                .arg("-f")
                .arg(&command.health_check_pattern)
                .current_dir(&command.cwd)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()?;
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&command.kill_command)
                .current_dir(&command.cwd)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()?;
        }
        Ok(())
    }

    fn toggle_command(&self, index: usize) {
        if let Some(command) = self.config.commands.get(index) {
            if self.is_command_running(&command.health_check_pattern) {
                _ = self.stop_command(command);
            } else {
                _ = self.start_command(command);
            }
        }
    }

    fn display_status(&self) {
        print!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1));
        io::stdout().flush().unwrap();

        println!("Services status:\n");

        for (index, command) in self.config.commands.iter().enumerate() {
            let indicator = if self.is_command_running(&command.health_check_pattern) {
                format!("{}●{}", color::Fg(color::Green), color::Fg(color::Reset))
            } else {
                format!("{}○{}", color::Fg(color::Red), color::Fg(color::Reset))
            };
            println!("   [{}] {} {}", index, indicator, command.name);
        }

        println!("\nPress [1..9] to toggle the services on/off.\nPress [r] to refresh or press [q] to quit.");
    }

    fn process_input(&self) -> bool {
        let stdin = io::stdin();
        let stdout = io::stdout()
            .into_raw_mode()
            .expect("Could not enter raw mode");

        if let Some(Ok(key)) = stdin.keys().next() {
            drop(stdout);

            match key {
                Key::Char('q') => {
                    println!("Exiting...");
                    return false;
                }
                Key::Char(c) => {
                    if c.is_numeric() {
                        if let Some(index) = c.to_digit(10) {
                            self.toggle_command(index as usize);
                        }
                    }
                }
                _ => {}
            }
        }

        true
    }

    pub fn run(&self) {
        loop {
            self.display_status();
            if !self.process_input() {
                break;
            }
        }
    }
}
