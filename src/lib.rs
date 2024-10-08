use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

use std::env;

use itertools::Itertools;

use flate2::write::GzEncoder;
use flate2::Compression;

pub const CHUNK_SIZE_DEFAULT: usize = 10;
pub const LEVEL_DEFAULT: Compression = Compression::fast();
pub const SEPARATOR_DEFAULT: u8 = b'\n';
pub const BUF_SIZE_DEFAULT: usize = 4096;

/// Lines -> Chunks -> Gzips -> Writer.
///
/// # Arguments
/// - lines: The iterator of lines(bytes).
/// - wtr: The target writer.
/// - buf: The buffer for gzip.
/// - chunk_size: The number of lines in a page(chunk).
/// - level: The compression level(e.g, fast, best, ...).
/// - sep: The "line" separator(e.g, '\n', '\0', ...).
pub fn lines2pages2gz2wtr<I, W>(
    lines: I,
    mut wtr: W,
    buf: &mut Vec<u8>,
    chunk_size: usize,
    level: Compression,
    sep: u8,
) -> Result<usize, io::Error>
where
    I: Iterator<Item = Vec<u8>>,
    W: Write,
{
    let chunks = lines.chunks(chunk_size);
    chunks.into_iter().try_fold(0, |state, chunk| {
        buf.clear();
        let mut enc = GzEncoder::new(buf.by_ref(), level);
        for nxt in chunk {
            let line: &[u8] = &nxt;
            enc.write_all(line)?;
            enc.write_all(&[sep])?;
        }
        enc.finish()?;
        wtr.write_all(buf)?;
		let sz: usize = buf.len();
		let tot: usize = state + sz;
		eprintln!("wrote count:{tot}");
        Ok(tot)
    })
}

pub fn rdr2gz2wtr<R, W>(
    rdr: R,
    mut wtr: W,
    buf: &mut Vec<u8>,
    chunk_size: usize,
    level: Compression,
    sep: u8,
) -> Result<usize, io::Error>
where
    R: Read,
    W: Write,
{
    let br = BufReader::new(rdr);
    let lines = br.split(sep);
    let noerr = lines.filter_map(|rslt| rslt.ok());

    let tot: usize = {
        let mut bw = BufWriter::new(wtr.by_ref());
        let cnt: usize = lines2pages2gz2wtr(noerr, bw.by_ref(), buf, chunk_size, level, sep)?;
        bw.flush()?;
        cnt
    };
    wtr.flush()?;
    Ok(tot)
}

pub fn stdin2gz2stdout(
    buf: &mut Vec<u8>,
    chunk_size: usize,
    level: Compression,
    sep: u8,
) -> Result<usize, io::Error> {
    let i = io::stdin();
    let il = i.lock();
    let o = io::stdout();
    let ol = o.lock();
    rdr2gz2wtr(il, ol, buf, chunk_size, level, sep)
}

pub fn stdin2gz2stdout_env() -> Result<usize, io::Error> {
    let buf_size: usize = env::var("ENV_BUF_SIZE")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(BUF_SIZE_DEFAULT);
    let chunk_size: usize = env::var("ENV_CHUNK_SIZE")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(CHUNK_SIZE_DEFAULT);
    let level: Compression = match env::var("ENV_GZ_LEVEL").ok() {
        None => LEVEL_DEFAULT,
        Some(s) => match s.as_str() {
            "best" => Compression::best(),
            "fast" => Compression::fast(),
            _ => Compression::fast(),
        },
    };
    let separator: u8 = env::var("ENV_SEPARATOR_BYTE")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(SEPARATOR_DEFAULT);
    let mut buf: Vec<u8> = Vec::with_capacity(buf_size);
    stdin2gz2stdout(&mut buf, chunk_size, level, separator)
}
