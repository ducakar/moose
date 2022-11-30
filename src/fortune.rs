use std::{
    convert::TryInto,
    fs,
    fs::File,
    io::{self, Read, Seek},
    mem,
    path::{Path, PathBuf},
};

use rand::Rng;

const FORTUNES_DIR: &str = "/usr/share/fortune";
const OFFENSIVE_FORTUNES_DIR: &str = "/usr/share/fortune/off";

pub struct Fortunes {
    fortunes: Vec<StrFile>,
}

impl Fortunes {
    pub fn load_database() -> io::Result<Fortunes> {
        let mut fortunes = Vec::new();
        Self::read_dir(FORTUNES_DIR, &mut fortunes)?;
        let _ = Self::read_dir(OFFENSIVE_FORTUNES_DIR, &mut fortunes);
        Ok(Fortunes { fortunes })
    }

    pub fn select(&self) -> io::Result<Option<String>> {
        if self.fortunes.is_empty() {
            Ok(None)
        } else {
            let mut rng = rand::thread_rng();
            let file_index = rng.gen_range(0..self.fortunes.len());
            match self.fortunes.get(file_index) {
                None => Ok(None),
                Some(strfile) => {
                    let ptr_index = rng.gen_range(0..strfile.pointers.len());
                    match Self::read_fragment(strfile, ptr_index) {
                        Ok(f) => Ok(Some(f)),
                        Err(e) => Err(e),
                    }
                }
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
        if strfile.is_encrypted {
            Self::decipher(&mut buffer)
        }

        let deciphered = String::from_utf8_lossy(&buffer).into_owned();
        Ok(deciphered)
    }

    fn decipher(bytes: &mut [u8]) {
        for b in bytes {
            if b.wrapping_sub(b'A') < 13 || b.wrapping_sub(b'a') < 13 {
                *b += 13;
            } else if b.wrapping_sub(b'N') < 13 || b.wrapping_sub(b'n') < 13 {
                *b -= 13;
            }
        }
    }
}

struct StrFile {
    path: PathBuf,
    is_encrypted: bool,
    pointers: Vec<u32>,
}

impl StrFile {
    fn load_from(dat_path: PathBuf) -> io::Result<StrFile> {
        let path = dat_path.with_extension("");
        let data = fs::read(dat_path)?;

        let n_strings = Self::read_u32_at(&data, 4)? as usize;
        let flags = Self::read_u32_at(&data, 16)?;
        let is_encrypted = (flags & 0x4) != 0;

        let mut pointers = Vec::with_capacity(n_strings);
        for i in 0..n_strings {
            let offset = 24 + i * mem::size_of::<u32>();
            let pointer = Self::read_u32_at(&data, offset)?;
            pointers.push(pointer);
        }

        Ok(StrFile {
            path,
            is_encrypted,
            pointers,
        })
    }

    fn read_u32_at(data: &[u8], index: usize) -> io::Result<u32> {
        let u32_bytes = data
            .get(index..index + 4)
            .ok_or(io::Error::from(io::ErrorKind::InvalidData))?;
        let u32_array = u32_bytes
            .try_into()
            .or(Err(io::Error::from(io::ErrorKind::InvalidData)))?;
        Ok(u32::from_be_bytes(u32_array))
    }
}
