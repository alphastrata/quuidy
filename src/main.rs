use clap::Parser;
use uuid::Uuid;

#[derive(Debug, clap::Parser)]
struct Cli {
    /// Number of uuids you want.
    #[clap(short, long)]
    count: u32,
    /// Do you want them printed in the Bevy #[Uuid = "your uuid here"] syntax.
    #[clap(short, long, default_value_t = false)]
    bevy: bool,
}

fn main() {
    let cli = Cli::parse();

    (0..cli.count).for_each(|_| {
        let uuid = Uuid::new_v4();
        if cli.bevy {
            println!("#[Uuid = \"{}\"]", uuid);
        } else {
            println!("{}", uuid);
        }
    });
}
