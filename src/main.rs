use clap::{Arg, Command};
use rand::prelude::*;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::str;

mod assets;

struct CowBubble {
    sleft: &'static str,
    sright: &'static str,
    topleft: &'static str,
    midleft: &'static str,
    botleft: &'static str,
    topright: &'static str,
    midright: &'static str,
    botright: &'static str,
}

fn list_cows() -> Vec<String> {
    assets::list()
        .iter()
        .map(|x| x.split("/").last().unwrap().replace(".cow", ""))
        .collect::<Vec<String>>()
}

fn format_animal(s: String, thoughts: &str, eyes: &str, tongue: &str) -> String {
    s.split("\n")
        .filter(|&x| !x.starts_with("##") && !x.contains("EOC"))
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .replace("$eyes", eyes)
        .replace("$thoughts", thoughts)
        .replace("$tongue", tongue)
        .replace("\\\\", "\\")
        .replace("\\@", "@")
}

fn make_bubble(s: String, width: usize, think: bool, wrap: bool) -> String {
    let mut result = Vec::new();
    let mut top = vec![" "];
    let mut bottom = vec![" "];
    let topc = "_";
    let bottomc = "-";
    let pad = ' ';
    let mut cowb = CowBubble {
        sleft: "<",
        sright: ">",
        topleft: "/",
        midleft: "|",
        botleft: "\\",
        topright: "\\",
        midright: "|",
        botright: "/",
    };

    if think {
        cowb = CowBubble {
            sleft: "(",
            sright: ")",
            topleft: "(",
            midleft: "(",
            botleft: "(",
            topright: ")",
            midright: ")",
            botright: ")",
        };
    }

    // Linewrap
    let mut index = 0;
    if wrap {
        loop {
            if index + width >= s.len() {
                break;
            }

            let localwidth;
            let mut subindex = index + width;
            'b: loop {
                match s[index..subindex].ends_with(" ") {
                    true => {
                        localwidth = subindex - index;
                        break 'b;
                    }
                    false => {
                        subindex -= 1;
                    }
                }
            }
            let slice = &s[index..index + localwidth];
            result.push(slice.to_string());
            index += localwidth;
        }
    }
    let slice = &s[index..];
    result.push(slice.to_string());

    // Bookend lines with bubble chars
    let mut longest = 0;
    let reslen = result.len() - 1;
    for (index, line) in result.iter_mut().enumerate() {
        match index {
            0 => match reslen {
                0 | 1 => *line = [cowb.sleft, line, cowb.sright].join(" "),
                _ => *line = [cowb.topleft, line, cowb.topright].join(" "),
            },
            x if x < reslen => *line = [cowb.midleft, line, cowb.midright].join(" "),
            y if y == reslen => match reslen {
                1 => *line = [cowb.sleft, line, cowb.sright].join(" "),
                _ => *line = [cowb.botleft, line, cowb.botright].join(" "),
            },
            _ => panic!("Whoops!"),
        }
        if line.len() > longest {
            longest = line.len();
        }
    }

    // Pad to longest line
    for line in &mut result {
        let mut padding = longest - line.len();
        let linelen = line.len();
        loop {
            match padding > 0 {
                false => break,
                true => {
                    line.insert(linelen - 1, pad);
                    padding -= 1;
                }
            };
        }
    }

    let mut top_bottom = longest - 2;
    loop {
        match top_bottom > 0 {
            false => break,
            true => {
                top.push(topc);
                bottom.push(bottomc);
                top_bottom -= 1;
            }
        }
    }
    result.insert(0, top.join(""));
    result.push(bottom.join(""));
    result.join("\n")
}

fn main() {
    let matches = Command::new("rust-cowsay")
        .version("v0.1.0-pre-alpha")
        .author("Matt Smith. <matthew.smith491@gmail.com>")
        .arg(
            Arg::new("MESSAGE")
                .help("Message for cow to say")
                .num_args(0..),
        )
        .arg(
            Arg::new("cow")
                .short('f')
                .value_name("COW")
                .help("Which cow should say"),
        )
        .arg(
            Arg::new("width")
                .short('W')
                .value_name("WIDTH")
                .help("Max width of cow text bubble"),
        )
        .arg(
            Arg::new("nowrap")
                .short('n')
                .action(clap::ArgAction::SetTrue)
                .help("Disable word wrap"),
        )
        .arg(
            Arg::new("borg")
                .short('b')
                .action(clap::ArgAction::SetTrue)
                .help("Borg Cow"),
        )
        .arg(
            Arg::new("dead")
                .short('d')
                .action(clap::ArgAction::SetTrue)
                .help("Dead Cow"),
        )
        .arg(
            Arg::new("greedy")
                .short('g')
                .action(clap::ArgAction::SetTrue)
                .help("Greedy Cow"),
        )
        .arg(
            Arg::new("paranoid")
                .short('p')
                .action(clap::ArgAction::SetTrue)
                .help("Paranoid Cow"),
        )
        .arg(
            Arg::new("stoned")
                .short('s')
                .action(clap::ArgAction::SetTrue)
                .help("Stoned Cow"),
        )
        .arg(
            Arg::new("tired")
                .short('t')
                .action(clap::ArgAction::SetTrue)
                .help("Tired Cow"),
        )
        .arg(
            Arg::new("wired")
                .short('w')
                .action(clap::ArgAction::SetTrue)
                .help("Wired Cow"),
        )
        .arg(
            Arg::new("youthful")
                .short('y')
                .action(clap::ArgAction::SetTrue)
                .help("Youthful Cow"),
        )
        .arg(
            Arg::new("custom")
                .short('e')
                .value_name("EYE_STRING")
                .help("Custom Eyes"),
        )
        .arg(
            Arg::new("tongue")
                .short('T')
                .value_name("TONGUE_STRING")
                .help("Custom Tongue"),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .action(clap::ArgAction::SetTrue)
                .help("List Cows"),
        )
        .arg(
            Arg::new("random")
                .long("random")
                .action(clap::ArgAction::SetTrue)
                .help("Choose random cow"),
        )
        .get_matches();

    if matches.get_flag("list") {
        let list = list_cows();
        println!("{list:?}");
        std::process::exit(0);
    };

    let mut cow = matches
        .get_one::<String>("cow")
        .cloned()
        .unwrap_or_else(|| "default".to_owned());

    cow = match matches.get_flag("random") {
        true => {
            let mut rng = rand::rng();
            let cows = list_cows();
            cows.choose(&mut rng).unwrap().to_owned()
        }
        false => cow,
    };

    let width = matches
        .get_one::<String>("width")
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap_or(40);
    let wrap = !matches.get_flag("nowrap");
    let message_vals: Vec<&str> = matches
        .get_many::<String>("MESSAGE")
        .map(|vals| vals.map(|s| s.as_str()).collect())
        .unwrap_or_else(|| vec![""]);
    let mut message = message_vals.join(" ");

    message = match &message[..] {
        "" => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            buffer.trim_end().to_string()
        }
        _ => message,
    };

    let tongue = matches
        .get_one::<String>("tongue")
        .map(|s| s.as_str())
        .unwrap_or(" ");

    // Cow Eyes
    let borg = matches.get_flag("borg");
    let dead = matches.get_flag("dead");
    let greedy = matches.get_flag("greedy");
    let paranoid = matches.get_flag("paranoid");
    let stoned = matches.get_flag("stoned");
    let tired = matches.get_flag("tired");
    let wired = matches.get_flag("wired");
    let youthful = matches.get_flag("youthful");
    let custom = matches
        .get_one::<String>("custom")
        .map(|s| s.as_str())
        .unwrap_or("");
    let mut custombool = false;

    if !custom.is_empty() {
        custombool = true;
    }

    let eyes = [
        (borg, "=="),
        (dead, "xx"),
        (greedy, "$$"),
        (paranoid, "@@"),
        (stoned, "**"),
        (tired, "--"),
        (wired, "OO"),
        (youthful, ".."),
        (custombool, custom),
        (true, "oo"),
    ]
    .iter()
    .filter(|&x| x.0)
    .collect::<Vec<_>>()[0]
        .1;

    let think;
    let voice;

    match env::args().collect::<Vec<_>>()[0].ends_with("cowthink") {
        true => {
            think = true;
            voice = "o"
        }
        false => {
            think = false;
            voice = "\\";
        }
    }

    let mut cowbody = String::new();

    match cow.contains(".cow") {
        true => {
            let mut f = File::open(&cow).unwrap();
            f.read_to_string(&mut cowbody)
                .unwrap_or_else(|_| panic!("Couldn't read cowfile {cow}"));
        }
        false => {
            let fmt = &format!("{}.cow", &cow);
            cowbody = str::from_utf8(&assets::get(fmt).unwrap())
                .unwrap()
                .to_string();
        }
    }

    println!("{}", make_bubble(message.to_string(), width, think, wrap));
    println!("{}", format_animal(cowbody, voice, eyes, tongue));
}
