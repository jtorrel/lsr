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
