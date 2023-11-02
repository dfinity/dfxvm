use console::{style, StyledObject};

pub fn style_command(cmd: &str) -> StyledObject<&str> {
    style(cmd).bold().cyan()
}
