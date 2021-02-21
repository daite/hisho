use clap::{Arg, App};
use hisho::{print_table, execute};

fn main() {
    let version = "0.1.0";
    let matches = App::new("search korean torrent magnet")
    .version(version)
    .author("daite <blueskykind02@yahoo.co.jp>")
    .about("search korean torrent magnet")
    .arg(Arg::with_name("keyword")
             .short("s")
             .long("search")
             .takes_value(true)
             .help("search korean torrent magnet file"))
    .get_matches();
    if let Some(keyword) = matches.value_of("keyword") {
        let keyword = keyword.to_owned();
        println!("[*] Hisho {} starts crawling!!!", version);
        let data = execute(keyword);
        if data.len() > 0 {
            print_table(data);
        }
    }
}