use fs2::FileExt;
use std::{ffi::OsString, fs::OpenOptions, io::Read};
use voidmap::{deserialize_screen, init_screen_log, Config, Screen};

mod cli;

fn is_binary(data: &[u8]) -> bool {
    // Check if data contains any non-UTF8 bytes
    String::from_utf8(data.to_vec()).is_err()
}

fn main() {
    // Initialise the CLI parser
    let app = cli::create();
    let matches = app.get_matches();

    // Handle version flag
    if matches.is_present("version") {
        println!("{} {}", cli::APP_NAME, cli::VERSION);
        return;
    }

    // Initialise screen logger
    init_screen_log().unwrap();

    let path: OsString = matches
        .value_of("PATH")
        .map(OsString::from)
        .or_else(|| {
            dirs::home_dir().and_then(|mut h| {
                h.push(".void.json");
                Some(h.into_os_string())
            })
        })
        .unwrap();

    // load from file if present
    let mut data = vec![];
    let mut f = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(&path)
        .unwrap();

    // exclusively lock the file
    f.try_lock_exclusive()
        .unwrap_or_else(|_| panic!("Another `void` process is using this path already!"));

    f.read_to_end(&mut data).unwrap();
    
    // Check if file is legacy binary format
    if !data.is_empty() && is_binary(&data) {
        eprintln!(
            "Error: Pre-1.2.0 binary database format detected at: {}\n\
            This is void ^1.2.0 which uses JSON format\n\
            To migrate your database, run:\n\
            ./scripts/migrate.sh {}\n\
            \nThis will create a JSON version you can use with void ^1.2.0",
            path.to_string_lossy(),
            path.to_string_lossy()
        );
        std::process::exit(1);
    }

    let saved_screen = if data.is_empty() {
        None
    } else {
        deserialize_screen(data).ok()
    };

    // Initialise the main working screen
    let mut screen = saved_screen.unwrap_or_else(Screen::default);

    screen.work_path = matches
        .value_of("PATH")
        .map(|s| s.into())
        .or_else(|| Some(path.into_string().unwrap()));

    if let Some(autosave_every) = matches
        .value_of("AUTOSAVE_EVERY")
        .and_then(|s| s.parse().ok())
    {
        screen.autosave_every = autosave_every;
    }

    let config = Config::maybe_parsed_from_env().unwrap();
    screen.config = config;

    screen.run();
}
