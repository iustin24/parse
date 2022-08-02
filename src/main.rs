use fluent_uri::Uri;
use fluent_uri::enc::EStr;
use std::env;
use std::io::{BufRead, stdin};
use anyhow::{Context, Result};


fn format(url: String, format: &str) -> Option<i32>{
    let parsed = Uri::parse(url.as_str()).ok()?;
    let authority = parsed.authority()?;
    let mut in_format = false;
    for l in format.chars() {
        if l == '%' && !in_format {
            in_format = true;
            continue
        }

        if !in_format {
            print!("{}", l);
            continue
        }
        match l {
            'd' => print!("{}",authority.host()),
            's' => print!("{}",parsed.scheme()?),
            'u' => print!("{}",authority.userinfo()?),
            'P' => print!("{}",authority.port()?),
            'p' => print!("{}",parsed.path()),
            'q' => print!("{}",parsed.query()?),
            'f' => print!("{}",parsed.fragment()?),
            'a' => print!("{}",authority),
            x => print!("{}",x)
        };

        in_format = false;
    }
    print!("{}","\n");
    Some(0)
}

fn main() {
    let fs = env::args().nth(1).unwrap();
    let output = stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap();
    for url in output {
        format(url, fs.as_str());
    }
}