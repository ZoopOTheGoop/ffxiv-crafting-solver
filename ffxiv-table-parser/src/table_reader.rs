#![allow(dead_code)]

use std::io::{self, Read};

/// A very slow skip line implementation that will not buffer over the line boundary.
/// This will panic if the internal reader fails for any reason, and for csv purposes
/// will technically fail if there's a newline inside a properly escaped string.
///
/// This is only meant to be used on the first few header lines in the tables, and *should*
/// only be used in that context due to these limitations.
fn skip_line<R: Read>(mut reader: R) {
    let mut buf = [0u8; 1];

    while buf[0] as char != '\n' {
        reader
            .read_exact(&mut buf)
            .expect("Did not expect error when skipping line");
    }
}

/// A very slow read line implementation that will not buffer over the line boundary.
/// This will panic if the internal reader fails for any reason, and for csv purposes
/// will technically fail if there's a newline inside a properly escaped string.
///
/// This is only meant to be used on the first few header lines in the tables, and *should*
/// only be used in that context due to these limitations.
fn fill_line<R: Read>(mut reader: R, linebuf: &mut Vec<u8>) {
    let mut buf = [0u8; 1];
    linebuf.clear();

    while buf[0] as char != '\n' {
        reader
            .read_exact(&mut buf)
            .expect("Did not expect error when filling line");
        linebuf.push(buf[0]);
    }
}

/// A simple adapter over a generic reader that makes it suitable for use with `csv::Reader`.
///
/// The datamined tables have three header rows, whereas `csv` only handles one. This adapter
/// simply filters out the two superfluous ones representing data types and raw numeric column keys.
#[derive(Clone, Debug)]
pub(crate) struct TableReader<R: Read> {
    reader: R,
    header: Vec<u8>,
}

impl<R: Read> TableReader<R> {
    /// Wraps the given reader, immediately consuming the first three lines and becoming
    /// suitable for use with [`csv::Reader`].
    pub(crate) fn new(mut reader: R) -> Self {
        skip_line(&mut reader);

        let mut header = Vec::new();
        fill_line(&mut reader, &mut header);

        skip_line(&mut reader);

        Self { reader, header }
    }
}

/// A basic no-underflow checked subtraction for unsigned integers.
fn diff_or_zero(a: usize, b: usize) -> usize {
    if a >= b {
        a - b
    } else {
        0
    }
}

impl<R: Read> Read for TableReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let leftover = if !self.header.is_empty() {
            let leftover = diff_or_zero(buf.len(), self.header.len());

            for (i, val) in self
                .header
                .drain(..buf.len().min(self.header.len()))
                .enumerate()
            {
                buf[i] = val;
            }

            leftover
        } else {
            0
        };

        if leftover > 0 {
            let buf_len = buf.len();
            let leftover_buf = &mut buf[buf_len - leftover..];

            match self.reader.read(leftover_buf) {
                Ok(size) => Ok(size + (buf.len() - leftover)),
                Err(e) => Err(e),
            }
        } else {
            Ok(buf.len())
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::{BufRead, BufReader};

    use super::TableReader;

    use csv::{ReaderBuilder, StringRecord};
    use serde::{Deserialize, Serialize};

    const TEST_HEADER: &str = r#"key,0,1,2
#,a,b,c,
int32,int32,int32,string
0,12,15,"hello",
"#;

    #[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
    struct TestType {
        a: i32,
        b: i32,
        c: String,
    }

    #[test]
    fn parses_header() {
        let reader = TableReader::new(TEST_HEADER.as_bytes());

        assert_eq!(
            reader.header,
            Vec::<u8>::from("#,a,b,c,\n"),
            "Parsed header is the wrong line"
        );

        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new();
        buf_reader
            .read_line(&mut line)
            .expect("Problem reading line");
        assert_eq!(line, "#,a,b,c,\n", "First line returned is not header");

        line.clear();
        buf_reader
            .read_line(&mut line)
            .expect("Problem reading line");

        assert_eq!(
            line, "0,12,15,\"hello\",\n",
            "First content line is not the proper string"
        );
    }

    #[test]
    fn csv_compat() {
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(TableReader::new(TEST_HEADER.as_bytes()));

        assert_eq!(
            &StringRecord::from(vec!["#", "a", "b", "c", ""]),
            reader.headers().expect("Error parsing headers")
        );

        let mut record = StringRecord::new();
        reader
            .read_record(&mut record)
            .expect("Could not get record");

        dbg!(&record);

        let record: TestType = record
            .deserialize(Some(reader.headers().expect("No headers?")))
            .expect("Could not decode first record");

        assert_eq!(
            record,
            TestType {
                a: 12,
                b: 15,
                c: "hello".into()
            }
        )
    }
}
