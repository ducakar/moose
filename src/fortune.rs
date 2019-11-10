use rand::prelude::*;
use serde::Deserialize;
use std::{
    fs,
    fs::File,
    io,
    io::{Read, Seek},
    mem,
    path::{Path, PathBuf},
};

const FORTUNES_DIR: &str = "/usr/share/fortune";
const OFFENSIVE_FORTUNES_DIR: &str = "/usr/share/fortune/off";

pub struct Fortunes {
    fortunes: Vec<StrFile>,
}

impl Fortunes {
    pub fn load_database() -> io::Result<Fortunes> {
        let mut fortunes = Vec::new();
        Self::read_dir(FORTUNES_DIR, &mut fortunes)?;
        Self::read_dir(OFFENSIVE_FORTUNES_DIR, &mut fortunes)?;
        Ok(Fortunes { fortunes })
    }

    pub fn select(&self) -> io::Result<Option<String>> {
        if self.fortunes.is_empty() {
            Ok(None)
        } else {
            let file_index = rand::thread_rng().next_u64() as usize;
            let ptr_index = rand::thread_rng().next_u64() as usize;
            match self.fortunes.get(file_index % self.fortunes.len()) {
                None => Ok(None),
                Some(sf) => match Self::read_fragment(sf, ptr_index) {
                    Ok(f) => Ok(Some(f)),
                    Err(e) => Err(e),
                },
            }
        }
    }

    fn read_dir<P: AsRef<Path>>(dir: P, fortunes: &mut Vec<StrFile>) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            match path.extension() {
                Some(ext) if ext == "dat" => {
                    let info = StrFile::load_from(path)?;
                    fortunes.push(info);
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn read_fragment(strfile: &StrFile, index: usize) -> io::Result<String> {
        let mut text_file = File::open(strfile.path.as_path())?;
        let text_file_len = text_file.metadata()?.len();
        if text_file_len > u32::max_value() as u64 {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }
        let text_file_len = text_file_len as u32;

        let index = index % strfile.pointers.len();
        let ptr = match strfile.pointers.get(index) {
            Some(&ptr) => ptr,
            None => return Err(io::Error::from(io::ErrorKind::UnexpectedEof)),
        };
        let next_ptr = strfile.pointers.get(index + 1);
        let size = match next_ptr {
            Some(&next_ptr) if ptr < next_ptr && next_ptr - ptr >= 3 => next_ptr - ptr - 3,
            None if ptr < text_file_len && text_file_len - ptr >= 2 => text_file_len - ptr - 2,
            _ => return Err(io::Error::from(io::ErrorKind::InvalidData)),
        };

        let offset = text_file.seek(io::SeekFrom::Start(ptr as u64))?;
        if offset != ptr as u64 {
            return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
        }

        let mut buffer = vec![0_u8; size as usize];
        text_file.read_exact(&mut buffer)?;
        if (strfile.header.flags & 0x4) != 0 {
            Self::decipher(&mut buffer)
        };

        let deciphered = String::from_utf8_lossy(&buffer).into_owned();
        Ok(deciphered)
    }

    fn decipher(encoded_bytes: &mut [u8]) {
        for b in encoded_bytes {
            if b'A' <= *b && *b <= b'Z' {
                *b = b'A' + (*b - b'A' + 13) % 26
            } else if b'a' <= *b && *b <= b'z' {
                *b = b'a' + (*b - b'a' + 13) % 26
            }
        }
    }
}

struct StrFile {
    path: PathBuf,
    header: Header,
    pointers: Vec<u32>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Header {
    version: u32,
    numstr: u32,
    longlen: u32,
    shortlen: u32,
    flags: u32,
    delim: u8,
}

impl StrFile {
    fn load_from(dat_path: PathBuf) -> io::Result<StrFile> {
        let mut config = bincode::config();
        let config = config.big_endian();
        let path = dat_path.with_extension("");
        let contents = fs::read(dat_path)?;

        let header: Header = match config.deserialize(&contents[..]) {
            Ok(h) => h,
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        };

        let mut pointers = Vec::new();
        let mut offset = mem::size_of::<Header>();
        for _ in 0..header.numstr {
            let begin: u32 = match config.deserialize(&contents[offset..]) {
                Ok(p) => p,
                Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
            };
            pointers.push(begin);
            offset += mem::size_of::<u32>();
        }

        Ok(StrFile {
            path,
            header,
            pointers,
        })
    }
}
