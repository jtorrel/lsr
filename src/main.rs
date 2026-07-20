use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: Option<std::path::PathBuf>,

    #[arg(short, long)]
    all: bool,
}

fn main() {
    let cli = Cli::parse();
    let arg_all: bool = cli.all;
    let selected_path = cli.path.unwrap_or_else(|| std::env::current_dir().unwrap());

    // Liste des fichiers dans le répertoire sélectionné
    let entries = lsr::fs::list(&selected_path, arg_all).unwrap();

    // Affichage des noms des fichiers
    for entry in entries {
        println!("{}", entry.name());
    }
}
