use std::io;
use std::io::Write;
use rayon::prelude::*;

pub struct Encoder<W: Write>(W);

impl<'a, W> Write for Encoder<W>
where
    W: Write + 'a,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(
            buf.par_iter()
                .flat_map(|byte| {
                    (0..8).into_par_iter()
                        .rev()
                        .map(move |pos| if byte >> pos as u8 & 1 == 0 {b'-'} else {b'\''})
                })
                .collect::<Vec<u8>>().as_slice()).map(|s| s / 8)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

fn main() {
    io::copy(&mut io::stdin(), &mut Encoder (io::stdout())).unwrap();
}
