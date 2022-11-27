use clap::{arg, command, Command};
use rand::Rng;
use walkdir::WalkDir;

// code comments are good for documentation so this is a comment uwu
fn get_files() -> Vec<String> {
    let mut files: Vec<String> = Vec::new();

    for entry in WalkDir::new("./src/") {
        let entry = entry.unwrap();

        let path = entry.path().to_str().unwrap();

        if path.ends_with(".rs") {
            files.push(path.to_string());
        }
    }

    files
}

// wow! such random! ඞ
fn random_chance() -> (bool, u8) {
    let mut rng = rand::thread_rng();

    // pretty random to me :)
    let random = 2;

    let imposter = rng.gen_range(1..=23);

    if random == 2 {
        (true, imposter)
    } else {
        (false, imposter)
    }
}

fn main() {
    let args = command!()
        .subcommand(
            Command::new("IAMTHEIMPOSTER")
                .about("Get rid of the other imposters by running the IMPOSTER-DETECTION-ALGORITHM")
                .arg(arg!([NAME])),
        )
        .get_matches();

    let files = get_files();

    let mut imposters = 0;

    if args.subcommand().is_none() {
        for path in files {
            let imp = populate_file(&path);
            imposters += imp;
        }

        println!("found {imposters} imposters in your code");
    } else {
        for path in files {
            loop {
                let imp = remove_imposters(&path);

                if imp == 0 {
                    break;
                }
            }
        }

        println!("eliminated all imposters from your sus code. it was a clean job");
    }
}

// OMG another comment!!!
fn populate_file(path: &str) -> u32 {
    let old_data = std::fs::read_to_string(path).unwrap();
    let new_data: &mut Vec<u8> = &mut Vec::new();

    let mut imposters = 0;

    for (index, character) in old_data.as_bytes().iter().enumerate() {
        new_data.push(*character);

        if character.to_ascii_lowercase() == 10 {
            if old_data.len() > index + 2 {
                if old_data.as_bytes()[index + 1] == 47 || old_data.as_bytes()[index + 2] == 47 {
                    continue;
                }
            } else if old_data.len() > index {
                let (chance, imposter) = random_chance();

                if chance {
                    imposters += 1;

                    add_imposter(new_data, imposter);
                }
            }
        }
    }

    std::fs::write(&path, new_data).unwrap();

    imposters
}

// ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ
fn remove_imposters(path: &str) -> u32 {
    let mut deleting: bool = false;

    let mut imposters = 0;

    let old_data = std::fs::read_to_string(path).unwrap();
    let new_data: &mut Vec<u8> = &mut Vec::new();

    for (index, character) in old_data.as_bytes().iter().enumerate() {
        if index > 5 {
            if old_data.len() > index + 4 {
                if old_data.as_bytes()[index + 1] == b'/'
                    && old_data.as_bytes()[index + 2] == b'/'
                    && old_data.as_bytes()[index + 3] == b'.'
                    && old_data.as_bytes()[index + 4] == b'?'
                {
                    imposters += 1;
                    deleting = true;
                }
            }

            if old_data.as_bytes()[index - 5] == b'/'
                && old_data.as_bytes()[index - 4] == b'/'
                && old_data.as_bytes()[index - 3] == b'?'
                && old_data.as_bytes()[index - 2] == b'.'
                && old_data.as_bytes()[index - 1] == b'\n'
            {
                deleting = false;
            }
        }

        if !deleting {
            new_data.push(*character);
        }
    }

    std::fs::write(&path, new_data).unwrap();
    imposters
}

// now the fun part!!! ඞ
fn add_imposter(new_data: &mut Vec<u8>, id: u8) {
    let imposter = match id {
        1 => r#""#,

        2 => r#""#,

        3 => r#""#,

        4 => r#""#,

        5 => r#""#,

        // THICCCCCCC!!!!!!!
        6 => r#""#,

        7 => r#""#,

        // THIICCCCCCCC!!!!
        8 => r#""#,

        9 => r#""#,

        10 => r#""#,

        11 => r#""#,

        12 => r#""#,

        13 => r#""#,

        14 => r#""#,

        15 => r#""#,

        16 => r#""#,

        17 => r#""#,

        18 => r#""#,

        19 => r#""#,

        20 => r#""#,

        21 => r#""#,

        // awwwwwwww!! ඞ
        22 => r#""#,

        23 => r#""#,

        _ => {
            r#"/*    
            ┼┼║┼┼ No case......
            */"#
        }
    };

    for i in imposter.chars() {
        let mut buf = [0; 5];

        let x = i.encode_utf8(&mut buf);

        for i in x.as_bytes() {
            new_data.push(*i);
        }
    }
}
