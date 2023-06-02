use std::{fs, io, path::Path};

const COW_DIR: &str = "/usr/share/cows";

pub struct Cow {
    pub pattern: String,
}

impl Cow {
    pub fn new(name: &str) -> io::Result<Cow> {
        let path = Path::new(COW_DIR).join(name).with_extension("cow");
        let content = fs::read_to_string(path)?;
        if let Some(begin) = content.find("$the_cow") {
            let content = &content[begin..];
            if let Some(begin) = content.find('\n') {
                let content = &content[begin..];
                if let Some(end) = content.rfind("\nEOC\n") {
                    let content = &content[..end];
                    let pattern = content.replace("\\\\", "\\");
                    return Ok(Cow { pattern });
                }
            }
        }
        Err(io::Error::from(io::ErrorKind::InvalidData))
    }

    pub fn print(&self, content: &str, thoughts: bool, eyes: &str, tongue: &str) -> String {
        Self::bubble(content, thoughts) + &self.avatar(thoughts, eyes, tongue)
    }

    fn bubble(text: &str, thoughts: bool) -> String {
        let text = text.replace('\t', "        ");
        match text.lines().map(|l| l.len()).max() {
            None => String::new(),
            Some(max_width) if thoughts => {
                let middle_lines: String = text
                    .lines()
                    .map(|l| format!("( {: <1$} )\n", l, max_width))
                    .collect();
                format!(
                    " _{empty:_<width$}_\n( {empty: <width$} )\n{middle}(_{empty:_<width$}_)",
                    empty = "",
                    width = max_width,
                    middle = middle_lines,
                )
            }
            Some(max_width) => {
                let middle_lines: String = text
                    .lines()
                    .map(|l| format!("| {: <1$} |\n", l, max_width))
                    .collect();
                format!(
                    " _{empty:_<width$}_\n/ {empty: <width$} \\\n{middle}\\_{empty:_<width$}_/",
                    empty = "",
                    width = max_width,
                    middle = middle_lines,
                )
            }
        }
    }

    fn avatar(&self, thoughts: bool, eyes: &str, tongue: &str) -> String {
        self.pattern
            .replace("$thoughts", if thoughts { "o" } else { "\\" })
            .replace("$eyes", eyes)
            .replace("$tongue", tongue)
    }
}
