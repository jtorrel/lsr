use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: Option<std::path::PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let selected_path = cli.path.unwrap_or_else(|| std::env::current_dir().unwrap());

    // Liste des fichiers dans le répertoire courant
    let entries = lsr::fs::list(&selected_path).unwrap();

    // Affichage des noms des fichiers
    for entry in entries {
        println!("{}", entry.name());
    }
}
