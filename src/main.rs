use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: Option<std::path::PathBuf>,

    #[arg(short, long)]
    all: bool,

    #[arg(short, long)]
    long: bool,
}

fn main() {
    let cli = Cli::parse();
    let arg_all: bool = cli.all;
    let arg_long: bool = cli.long;
    let selected_path = cli.path.unwrap_or_else(|| std::env::current_dir().unwrap());

    // Liste des fichiers dans le répertoire sélectionné
    let entries = lsr::fs::list(&selected_path, arg_all).unwrap();

    if arg_long {
        println!(
            "{:<30} \t{:<10} \t{:<30} \t{:<20} \t{:<10}\n{:-<113}",
            "Name", "Kind", "Size", "Modified", "Read-only", "-"
        );
    }

    // Affichage des noms des fichiers
    for entry in entries {
        println!("{}", entry.format_entry(arg_long));
    }
}
