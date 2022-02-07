use std::io::{stdin, stdout, BufWriter, Read, Write, BufReader};

struct Args {
    pub pretty: bool,
}

fn main() -> anyhow::Result<()> {
    let args = parse_args();

    let obj = {
        let mut buf = String::new();
        let stdin = stdin();
        let mut buf_in = BufReader::new(stdin.lock());
        buf_in.read_to_string(&mut buf).unwrap();

        serde_json::from_str::<serde_json::Value>(&buf)
    }?;

    let sorted = sort_json(&obj, &args)?;

    {
        let stdout = stdout();
        let mut out = BufWriter::new(stdout.lock());
        writeln!(out, "{}", sorted).unwrap();
    }

    Ok(())
}

fn parse_args() -> Args {
    let matches = clap::App::new("sort-json")
        .about("Sort json keys")
        .arg(
            clap::Arg::new("pretty")
                .short('p')
                .help("Output pretty json"),
        )
        .get_matches();

    Args {
        pretty: matches.is_present("pretty"),
    }
}

fn sort_json(obj: &serde_json::Value, args: &Args) -> anyhow::Result<String> {
    let s = if args.pretty {
        serde_json::to_string_pretty(&obj)?
    } else {
        serde_json::to_string(&obj)?
    };
    Ok(s)
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sort_json() {
        let obj = json!({
            "ccc": 3,
            "aaa": 1,
            "ddd": [4, 4],
            "bbb": 2,
        });

        // Simple
        {
            let expected = serde_json::to_string(&json!({
                "aaa": 1,
                "bbb": 2,
                "ccc": 3,
                "ddd": [4, 4],
            }))
            .unwrap();

            let args = Args { pretty: false };

            let sorted = sort_json(&obj, &args).unwrap();
            assert_eq!(sorted, expected);
        }

        // Pretty
        {
            let expected = serde_json::to_string_pretty(&json!({
                "aaa": 1,
                "bbb": 2,
                "ccc": 3,
                "ddd": [4, 4],
            }))
            .unwrap();

            let args = Args { pretty: true };

            let sorted = sort_json(&obj, &args).unwrap();
            assert_eq!(sorted, expected);
        }
    }
}
