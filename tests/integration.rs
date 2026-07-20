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
    let entries: Vec<Entry> = lsr::fs::list(dir.path(), false).unwrap();

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
    let result = lsr::fs::list(&file_path, false);
    assert!(result.is_err());
}

#[test]
fn test_list_hidden_files() {
    // Répertoire temporaire pour les tests
    let dir = tempdir().unwrap();

    // Création de fichiers temporaires dans le répertoire
    std::fs::File::create(dir.path().join("foo.txt")).unwrap();
    std::fs::File::create(dir.path().join(".hidden.txt")).unwrap();

    // Récupération des entrées du répertoire temporaire sans l'option "all"
    let entries_without_all: Vec<Entry> = lsr::fs::list(dir.path(), false).unwrap();
    assert_eq!(entries_without_all.len(), 1);
    assert!(entries_without_all.iter().any(|e| e.name() == "foo.txt"));
    assert!(
        !entries_without_all
            .iter()
            .any(|e| e.name() == ".hidden.txt")
    );

    // Récupération des entrées du répertoire temporaire avec l'option "all"
    let entries_with_all: Vec<Entry> = lsr::fs::list(dir.path(), true).unwrap();
    assert_eq!(entries_with_all.len(), 2);
    assert!(entries_with_all.iter().any(|e| e.name() == "foo.txt"));
    assert!(entries_with_all.iter().any(|e| e.name() == ".hidden.txt"));
}
