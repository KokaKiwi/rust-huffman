use std::io::{BufferedReader, BufferedWriter};

pub struct BitsWriter<W: Writer> {
    inner: W,
}
