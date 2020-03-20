use adler32::adler32;
use anyhow::{ensure, Result};

use std::{
    fmt,
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
    path::Path,
};

#[derive(Debug)]
pub struct Diff {
    offset: u64,
    reference: String,
    modified: String,
}

impl Diff {
    fn new(offset: u64, a: &[u8], b: &[u8]) -> Result<Self> {
        Ok(Self {
            offset,
            reference: String::from_utf8(a.to_owned())?,
            modified: String::from_utf8(b.to_owned())?,
        })
    }
}

#[derive(Default)]
pub struct Diffs(Vec<Diff>);

impl fmt::Display for Diffs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            write!(f, "the files match")
        } else {
            writeln!(f, "the files are different ({} differences):", self.0.len())?;
            for diff in &self.0 {
                writeln!(f, "{:?}", diff)?;
            }
            Ok(())
        }
    }
}

pub fn compare_files<T: AsRef<Path>>(path1: T, path2: T, chunk_size: usize) -> Result<Diffs> {
    ensure!(chunk_size > 0, "chunk size must be greater than 0");

    let (mut file1, mut file2) = (File::open(path1)?, File::open(path2)?);
    let mut diffs = Diffs::default();
    let mut buf1 = vec![0; chunk_size];
    let mut buf2 = buf1.clone();

    let (mut adler1, mut adler2);
    while let Ok(read1) = file1.read(&mut buf1) {
        if read1 == 0 {
            break;
        }

        adler1 = adler32(Cursor::new(&buf1[..read1]))?;

        if let Ok(read2) = file2.read(&mut buf2) {
            adler2 = adler32(Cursor::new(&buf2[..read2]))?;

            if adler1 != adler2 {
                let offset = file1.seek(SeekFrom::Current(0))?;
                let diff = Diff::new(offset, &buf1[..read1], &buf2[..read2])?;
                diffs.0.push(diff);
            }

            if read2 == 0 {
                break;
            }
        }
    }

    Ok(diffs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_diff() -> Result<()> {
        let mut toml = env!("CARGO_MANIFEST_DIR").to_owned();
        toml.push_str("/Cargo.toml");

        for n in 1..64 {
            assert!(compare_files(&toml, &toml, n)?.0.is_empty());
        }

        Ok(())
    }

    #[test]
    fn diffs() -> Result<()> {
        let mut toml = env!("CARGO_MANIFEST_DIR").to_owned();
        toml.push_str("/Cargo.toml");
        let mut lock = env!("CARGO_MANIFEST_DIR").to_owned();
        lock.push_str("/Cargo.lock");

        for n in 1..64 {
            assert!(!compare_files(&toml, &lock, n)?.0.is_empty());
        }

        Ok(())
    }
}
