use std::{path::PathBuf, io::BufRead};

use clap::Parser;

/// Gaji interpreter
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the gaji source to run
    input: Option<PathBuf>,
    /// If set, print the code to stderr before run
    #[arg(short, long)]
    echo: bool,
}

fn main() {
    let args = Args::parse();
    let mut reader = BufReader::new(stdin().lock());
    let source = if let Some(input) = args.input {
        std::fs::read_to_string(input).expect("cannot read file")
    } else {
        let mut lines = String::new();
        loop {
            reader.read_line(&mut lines).expect("cannot read from stdin");
            if lines.ends_with("\n\n") {
                lines.truncate(lines.len() - 2);
                break lines;
            }
        }
    };
    let mut row = 0;
    let mut col = 0;
    for line in source.lines() {
        row += 1;
        col = col.max(
            line.chars()
                .rev()
                .skip_while(|&c| c != '🍆' && c != '🌱')
                .count(),
        );
    }
    let mut area = vec![Field::Ignore; row * col];
    let mut r = 0;
    let mut c = 0;
    for ch in source.chars() {
        match ch {
            '🍆' => {
                area[r * col + c] = Field::Eggplant;
                c += 1;
            }
            '🌱' => {
                area[r * col + c] = Field::Empty;
                c += 1;
            }
            '\n' => {
                r += 1;
                c = 0;
            }
            // Field::Ignore
            _ => {
                c += 1;
            }
        };
    }
    if args.echo {
        debug(col, &area);
    }
    use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};
    let mut writer = BufWriter::new(stdout().lock());
    let mut r = 0;
    let mut c = 0;
    let mut dr = 0;
    let mut dc = 1;
    let mut bit_out_buf = [0u8];
    let mut bit_out_cnt = 0;
    let mut bit_in_buf = [0u8];
    let mut bit_in_cnt = 8;
    let mut not_place = false;
    let mut not_harvest = false;
    let mut not_execute = false;
    while r < row && c < col {
        match area[r * col + c] {
            Field::Empty if not_place => {
                not_place = false;
            }
            Field::Empty => {
                area[r * col + c] = Field::Eggplant;
            }
            Field::Eggplant if not_harvest => {
                not_harvest = false;
            }
            Field::Eggplant => {
                area[r * col + c] = Field::Empty;
                if not_execute {
                    not_execute = false;
                } else {
                    let mut mul = 1;
                    let mut x = 0;
                    for dr in [1usize.wrapping_neg(), 0, 1] {
                        for dc in [1usize.wrapping_neg(), 0, 1] {
                            if dr == 0 && dc == 0 {
                                continue;
                            }
                            let nr = r.wrapping_add(dr);
                            let nc = c.wrapping_add(dc);
                            if nr < row && nc < col {
                                x = (x + mul * area[nr * col + nc].value()) % 7;
                            }
                            mul = mul * 2 % 7;
                        }
                    }
                    match x {
                        0 => {
                            let sr = r.wrapping_add(dr);
                            let sc = c.wrapping_add(dc);
                            if sr < row && sc < col {
                                let byte = &mut bit_out_buf[0];
                                *byte = (*byte << 1) | area[sr * col + sc].value() as u8;
                                bit_out_cnt += 1;
                                if bit_out_cnt == 8 {
                                    writer
                                        .write(&mut bit_out_buf)
                                        .expect("cannot write to stdout");
                                    bit_out_cnt = 0;
                                }
                            }
                        }
                        1 => {
                            (dr, dc) = (dc.wrapping_neg(), dr);
                        }
                        2 => {
                            (dr, dc) = (dc, dr.wrapping_neg());
                        }
                        3 => {
                            not_place = true;
                        }
                        4 => {
                            not_harvest = true;
                        }
                        5 => {
                            not_execute = true;
                        }
                        6 => {
                            let mut read = true;
                            if bit_in_cnt == 8 {
                                let len = reader
                                    .read(&mut bit_in_buf)
                                    .expect("cannot read from stdin");
                                if len == 0 {
                                    read = false;
                                }
                            }
                            if read {
                                if bit_in_buf[0] & 1 == 1 {
                                    area[r * col + c] = Field::Eggplant;
                                }
                                bit_in_buf[0] >>= 1;
                                bit_in_cnt += 1;
                            }
                        }
                        _ => unreachable!("x >= 7"),
                    }
                }
            }
            Field::Ignore => {}
        }
        // debug(col, &area);
        r = r.wrapping_add(dr);
        c = c.wrapping_add(dc);
    }
    writer.flush().expect("cannot write to stdout");
}

fn debug(col: usize, area: &[Field]) {
    let mut s = String::new();
    for row in area.chunks(col) {
        for f in row {
            let c = match f {
                Field::Eggplant => '🍆',
                Field::Empty => '🌱',
                Field::Ignore => '❓',
            };
            s.push(c);
        }
        s.push('\n');
    }
    eprintln!("code:");
    eprintln!("{}", s);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Field {
    Eggplant,
    Empty,
    Ignore,
}

impl Field {
    fn value(&self) -> usize {
        match self {
            Self::Eggplant => 1,
            _ => 0,
        }
    }
}
