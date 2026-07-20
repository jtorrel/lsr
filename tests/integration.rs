use lsr::entry::Entry;
use tempfile::tempdir;

#[test]
fn test_list_files() {
    // Répertoire temporaire pour les tests
    let dir = tempdir().unwrap();

    // Création de fichiers temporaires dans le répertoire
    std::fs::File::create(dir.path().join("foo.txt")).unwrap();
    std::fs::File::create(dir.path().join("bar.txt")).unwrap();

    // Récupération des entrées du répertoire temporaire
    let entries: Vec<Entry> = lsr::fs::list(dir.path()).unwrap();

    assert_eq!(entries.len(), 2);
    assert!(entries.iter().any(|e| e.name() == "foo.txt"));
    assert!(entries.iter().any(|e| e.name() == "bar.txt"));
}

#[test]
fn test_list_non_directory() {
    // Création d'un fichier temporaire
    let file_path = std::env::temp_dir().join("temp_file.txt");
    std::fs::File::create(&file_path).unwrap();

    // Tentative de lister les fichiers dans un fichier
    let result = lsr::fs::list(&file_path);
    assert!(result.is_err());
}
