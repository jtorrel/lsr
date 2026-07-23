use lsr::entry::Entry;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;
use tempfile::tempdir;

#[test]
fn test_list_files() {
    // Répertoire temporaire pour les tests
    let dir = tempdir().unwrap();

    // Création de fichiers temporaires dans le répertoire
    File::create(dir.path().join("foo.txt")).unwrap();
    File::create(dir.path().join("bar.txt")).unwrap();

    // Récupération des entrées du répertoire temporaire
    let entries: Vec<Entry> = lsr::fs::list(
        dir.path(),
        &lsr::options::Options {
            all: false,
            recursive: false,
        },
    )
    .unwrap();

    assert_eq!(entries.len(), 2);
    assert!(entries.iter().any(|e| e.name() == "foo.txt"));
    assert!(entries.iter().any(|e| e.name() == "bar.txt"));
}

#[test]
fn test_list_non_directory() {
    // Création d'un fichier temporaire
    let file_path = std::env::temp_dir().join("temp_file.txt");
    File::create(&file_path).unwrap();

    // Tentative de lister les fichiers dans un fichier
    let result = lsr::fs::list(
        &file_path,
        &lsr::options::Options {
            all: false,
            recursive: false,
        },
    );
    assert!(result.is_err());
}

#[test]
fn test_list_hidden_files() {
    // Répertoire temporaire pour les tests
    let dir: TempDir = tempdir().unwrap();

    // Création de fichiers temporaires dans le répertoire
    File::create(dir.path().join("foo.txt")).unwrap();
    File::create(dir.path().join(".hidden.txt")).unwrap();

    // Récupération des entrées du répertoire temporaire sans l'option "all"
    let entries_without_all: Vec<Entry> = lsr::fs::list(
        dir.path(),
        &lsr::options::Options {
            all: false,
            recursive: false,
        },
    )
    .unwrap();
    assert_eq!(entries_without_all.len(), 1);
    assert!(entries_without_all.iter().any(|e| e.name() == "foo.txt"));
    assert!(
        !entries_without_all
            .iter()
            .any(|e| e.name() == ".hidden.txt")
    );

    // Récupération des entrées du répertoire temporaire avec l'option "all"
    let entries_with_all: Vec<Entry> = lsr::fs::list(
        dir.path(),
        &lsr::options::Options {
            all: true,
            recursive: false,
        },
    )
    .unwrap();
    assert_eq!(entries_with_all.len(), 2);
    assert!(entries_with_all.iter().any(|e| e.name() == "foo.txt"));
    assert!(entries_with_all.iter().any(|e| e.name() == ".hidden.txt"));
}

#[test]
fn test_list_entrykind_count() {
    // Répertoires temporaires
    let dir: TempDir = tempdir().unwrap();
    let subdir = dir.path().join("subdir");
    std::fs::create_dir(&subdir).unwrap();

    // Création de fichiers temporaires dans le répertoire
    File::create(subdir.join("foo.txt")).unwrap();
    File::create(subdir.join("bar.txt")).unwrap();

    // Récupération des entrées du répertoire temporaire
    let entries: Vec<Entry> = lsr::fs::list(
        dir.path(),
        &lsr::options::Options {
            all: false,
            recursive: false,
        },
    )
    .unwrap();

    // Vérification du type et du nombre d'entrées pour le sous-répertoire
    for entry in entries {
        if entry.name() == "subdir" {
            match entry.kind() {
                lsr::entry::EntryKind::Directory { count } => {
                    assert_eq!(*count, 2); // Le sous-répertoire contient 2 fichiers
                }
                _ => panic!("Expected a directory entry"),
            }
        }
    }
}

#[test]
fn test_list_entrykind_size() {
    // Répertoire temporaire pour les tests
    let dir: TempDir = tempdir().unwrap();

    // Création de fichiers temporaires dans le répertoire
    {
        let mut foo_file: File = File::create(dir.path().join("foo.txt")).unwrap();
        foo_file.write_all(b"hello").unwrap();
    } // Drop pour s'assurer que le fichier est écrit avant de continuer

    // Récupération des entrées du répertoire temporaire
    let entries: Vec<Entry> = lsr::fs::list(
        dir.path(),
        &lsr::options::Options {
            all: false,
            recursive: false,
        },
    )
    .unwrap();

    // Vérification du type et de la taille des entrées
    for entry in entries {
        match entry.kind() {
            lsr::entry::EntryKind::File { size } => {
                assert_eq!(*size, 5); // Le fichier foo.txt a une taille de 5 octets
            }
            _ => panic!("Expected a file entry"),
        }
    }
}

#[test]
fn test_list_recursive_directories() {
    // Répertoire temporaire pour les tests
    let dir: TempDir = tempdir().unwrap();

    // Création de l'arborescence de répertoires
    let subdir1 = dir.path().join("subdir1");
    std::fs::create_dir(&subdir1).unwrap();
    let subdir2 = subdir1.join("subdir2");
    std::fs::create_dir(&subdir2).unwrap();
    let subdir11: PathBuf = subdir1.join("subdir11");
    std::fs::create_dir(&subdir11).unwrap();
    let subdir12: PathBuf = subdir1.join("subdir12");
    std::fs::create_dir(&subdir12).unwrap();

    // Récupération des entrées du répertoire temporaire
    let entries: Vec<Entry> = lsr::fs::list(
        dir.path(),
        &lsr::options::Options {
            all: false,
            recursive: true,
        },
    )
    .unwrap();

    // Vérification de la présence des sous-sous-répertoires
    assert!(entries.iter().any(|e| e.name() == "subdir11"));
    assert!(entries.iter().any(|e| e.name() == "subdir12"));
}
