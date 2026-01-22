# 🦀 Les Bases de Cargo - Guide de Démarrage Rapide

Ce guide couvre **uniquement les fondamentaux** pour démarrer rapidement avec Cargo. Il reste volontairement simple ; si un point manque, la [documentation officielle](https://doc.rust-lang.org/cargo/) complètera.

## 📦 Qu'est-ce que Cargo ?

Cargo est l'outil de build et le gestionnaire de paquets de Rust. Il gère :

- ✅ La compilation de votre code
- ✅ Le téléchargement des dépendances
- ✅ L'exécution des tests
- ✅ La génération de documentation

## 🚀 Commandes essentielles

### Nettoyage

```bash
cargo clean          # Supprime le dossier target (cache de compilation)
```

**🎯 À quoi ça sert ?** Libérer de l'espace disque ou forcer une recompilation complète en cas de problème bizarre.

### Vérification et compilation

```bash
cargo check          # Vérifie que le code compile (rapide, sans binaire)
cargo build          # Compile en mode debug
cargo build --release # Compile en mode release (optimisé, plus lent)
```

**💡 Astuce** : `cargo check` est **10x plus rapide** que `cargo build` pour vérifier les erreurs.

**🎯 À quoi ça sert ?**

- `check` : Détecter les erreurs de compilation sans perdre de temps à générer le binaire
- `build` : Produire un exécutable pour tester localement
- `build --release` : Créer un binaire optimisé pour la production

### Exécution

```bash
cargo run            # Compile et exécute
cargo run --release  # Exécute la version optimisée
cargo run -- --arg   # Passe des arguments à votre programme
```

**🎯 À quoi ça sert ?** Tester rapidement votre application sans compiler manuellement puis exécuter.

### Tests

```bash
cargo test           # Exécute tous les tests
cargo test test_name # Exécute un test spécifique
cargo test -- --nocapture # Affiche stdout pendant les tests
cargo test -- --test-threads=1 # Tests en séquentiel (debugging)
```

**🎯 À quoi ça sert ?** Valider que votre code fonctionne correctement, éviter les régressions.

### Formatage et qualité

```bash
# Formatage automatique du code
cargo fmt            # Formate tous les fichiers
cargo fmt -- --check # Vérifie sans modifier (CI/CD)

# Analyse statique (linter)
cargo clippy         # Détecte 600+ anti-patterns et bugs potentiels
cargo clippy --fix   # Corrige automatiquement les problèmes simples
cargo clippy -- -D warnings  # Traite les warnings comme des erreurs
cargo clippy -- -W clippy::all # Active tous les lints

# Auto-corrections du compilateur
cargo fix            # Applique les suggestions de rustc
cargo fix --edition  # Migre vers une nouvelle édition Rust
```

**⚠️ Important** : Lancez `cargo clippy` avant chaque commit !

**🎯 À quoi ça sert ?**

- `fmt` : Style de code uniforme (indentation, espacement, etc.)
- `clippy` : Détecte les bugs, problèmes de performance, code non-idiomatique
- `fix` : Applique automatiquement les corrections suggérées par le compilateur

**🔧 Comment fixer les problèmes** :

```bash
# Workflow de correction automatique
cargo fmt              # 1. Formater le code
cargo fix              # 2. Appliquer les corrections rustc
cargo clippy --fix     # 3. Appliquer les corrections clippy
cargo clippy -- -D warnings # 4. Vérifier qu'il ne reste aucun warning
```

### Documentation

```bash
cargo doc --open     # Génère et ouvre la doc dans le navigateur
cargo doc --no-deps  # Sans la doc des dépendances
```

**🎯 À quoi ça sert ?** Générer une documentation HTML navigable à partir des commentaires `///` dans votre code.

**📄 Ce que ça génère exactement** :

- Un dossier HTML dans `target/doc/`.
- La page d'entrée principale se trouve en général à :
  - `target/doc/<nom_du_crate>/index.html`
  - Exemple pour ce projet : [target/doc/battery_charge_manager/index.html](target/doc/battery_charge_manager/index.html)

**🧭 Selon le type de crate** :

- Si vous avez une bibliothèque (`lib.rs`), l'entrée est généralement dans `target/doc/<nom_du_crate>/index.html`.
- Si vous avez uniquement un binaire (`main.rs`), l'entrée peut se trouver dans `target/doc/<nom_du_binaire>/index.html`.
- En cas de doute, ouvrez `target/doc/index.html` et naviguez depuis la liste.

**✅ Comment l'utiliser** :

- Ouvrir directement la page avec un navigateur.
- Ou lancer :

```bash
cargo doc --open
```

Astuce : si vous ne voulez pas la doc des dépendances, utilisez `cargo doc --no-deps` pour aller plus vite.

## 🗂️ Structure d'un Projet Cargo

```
mon-projet/
├── Cargo.toml       # Manifeste du projet (dépendances, métadonnées)
├── Cargo.lock       # Versions exactes des dépendances (généré auto)
├── src/
│   ├── main.rs      # Point d'entrée pour un binaire
│   ├── lib.rs       # Point d'entrée pour une bibliothèque
│   └── bin/         # Binaires secondaires
└── target/          # Dossier de build (généré, à ignorer en git)
```

## 📋 Cargo.toml de base

```toml
[package]
name = "mon-projet"
version = "0.1.0"
edition = "2021"

[dependencies]
# Format: nom_crate = "version"
serde = "1.0"
gtk4 = "0.7"

[dev-dependencies]
# Dépendances uniquement pour les tests
mockall = "0.12"
```

**Versions** :

- `"1.0"` → Compatible avec 1.x (sémantique: `^1.0`)
- `"=1.0.5"` → Exactement 1.0.5
- `">=1.0, <2.0"` → Plage spécifique

## 🔧 Workflow de Développement Quotidien

```bash
# 1. Vérifier rapidement
cargo check

# 2. Corriger le style
cargo fmt

# 3. Analyser le code
cargo clippy

# 4. Lancer les tests
cargo test

# 5. Exécuter l'application
cargo run
```

## 🛠️ Outils Cargo intégrés (déjà installés)

Je garde cette section simple et concise. Si vous avez besoin de précision, la documentation officielle complétera.

| Outil          | À quoi ça sert (simple)                          | Commandes utiles (et rôle)                                                                                                       |
| -------------- | ------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------- |
| `cargo check`  | Vérifie la compilation sans produire de binaire. | `cargo check` — rapide pour itérer.                                                                                              |
| `cargo build`  | Compile le binaire en debug.                     | `cargo build` — debug par défaut.<br>`cargo build --release` — version optimisée.                                                |
| `cargo run`    | Compile puis exécute le programme.               | `cargo run` — exécution standard.<br>`cargo run --release` — exécution optimisée.<br>`cargo run -- --arg` — passe des arguments. |
| `cargo test`   | Lance les tests.                                 | `cargo test` — tout exécuter.<br>`cargo test test_name` — cibler un test.<br>`cargo test -- --nocapture` — afficher stdout.      |
| `cargo fmt`    | Formate le code.                                 | `cargo fmt` — formatage local.<br>`cargo fmt -- --check` — vérif en CI.                                                          |
| `cargo clippy` | Linter (qualité du code).                        | `cargo clippy` — analyse.<br>`cargo clippy --fix` — corrections simples.<br>`cargo clippy -- -D warnings` — warnings en erreurs. |
| `cargo doc`    | Génère la documentation HTML.                    | `cargo doc --open` — ouvre la doc.<br>`cargo doc --no-deps` — sans dépendances.                                                  |
| `cargo tree`   | Affiche l'arbre des dépendances.                 | `cargo tree` — vue globale.                                                                                                      |
| `cargo clean`  | Nettoie le dossier `target`.                     | `cargo clean` — rebuild propre.                                                                                                  |
| `cargo update` | Met à jour `Cargo.lock`.                         | `cargo update` — global.<br>`cargo update -p serde` — ciblé.                                                                     |
| `cargo fix`    | Applique les suggestions de `rustc`.             | `cargo fix` — corrections auto.<br>`cargo fix --edition` — migration d’édition.                                                  |

Workflow de base (rapide) :

```bash
cargo check
cargo fmt
cargo fix
cargo clippy -- -D warnings
cargo test
cargo run
```

## 📦 Outils Cargo avancés (à installer)

Je garde un format simple et régulier : **ce que fait l'outil**, **comment l'installer**, puis **2–3 commandes utiles**. Si je simplifie trop, la doc officielle complètera.

### Sécurité

#### `cargo-audit`

- **Sert à** : détecter les vulnérabilités connues (CVE) dans les dépendances.
- **Installer** : `cargo install cargo-audit`
- **Commandes utiles** :
  - `cargo audit` — liste les vulnérabilités trouvées.

#### `cargo-deny`

- **Sert à** : vérifier licences, sources et règles de sécurité.
- **Installer** : `cargo install cargo-deny`
- **Commandes utiles** :
  - `cargo deny check` — lance les contrôles configurés.

#### `cargo-geiger`

- **Sert à** : compter le code `unsafe` dans le projet et ses dépendances.
- **Installer** : `cargo install cargo-geiger`
- **Commandes utiles** :
  - `cargo geiger` — affiche le rapport `unsafe`.

### Dépendances

#### `cargo-outdated`

- **Sert à** : voir quelles dépendances sont obsolètes.
- **Installer** : `cargo install cargo-outdated`
- **Commandes utiles** :
  - `cargo outdated` — liste complète.
  - `cargo outdated -R` — dépendances directes uniquement.

#### `cargo-edit`

- **Sert à** : ajouter/retirer/mettre à jour des dépendances via CLI.
- **Installer** : `cargo install cargo-edit`
- **Commandes utiles** :
  - `cargo add serde` — ajoute une dépendance.
  - `cargo add --dev serde` — ajoute en dev‑dependencies.
  - `cargo add --dry-run serde` — simule l'ajout.
  - `cargo rm serde` — retire une dépendance.
  - `cargo upgrade` — met à jour selon les contraintes.

### Taille et performance

#### `cargo-bloat`

- **Sert à** : analyser ce qui alourdit le binaire.
- **Installer** : `cargo install cargo-bloat`
- **Commandes utiles** :
  - `cargo bloat --release` — top des symboles volumineux.
  - `cargo bloat --release --crates` — analyse par crate.

### Documentation

#### `cargo-rdme`

- **Sert à** : générer/valider un README à partir des doc-comments.
- **Installer** : `cargo install cargo-rdme`
- **Commandes utiles** :
  - `cargo rdme --check` — vérifie que le README est à jour.

#### `cargo-deadlinks`

- **Sert à** : détecter les liens morts dans la doc.
- **Installer** : `cargo install cargo-deadlinks`
- **Commandes utiles** :
  - `cargo deadlinks` — cherche les liens cassés.

### Tests avancés

#### `miri`

- **Sert à** : détecter les comportements indéfinis dans les tests.
- **Installer** : `cargo install miri` (un setup complémentaire peut être nécessaire selon la plateforme).
- **Commandes utiles** :
  - `cargo miri test` — exécute les tests sous Miri.
  - `cargo miri run` — exécute le binaire sous Miri.

### Installation groupée (optionnel)

```bash
cargo install cargo-audit cargo-deny cargo-geiger \
              cargo-outdated cargo-edit cargo-bloat \
              cargo-rdme cargo-deadlinks miri
```

Conseils pratiques :

- Ajoutez la liste des outils requis et leurs versions dans votre `README.md`.
- En CI, préférez `cargo install --locked` et mettez en cache `~/.cargo/bin` et `~/.cargo/registry`.

## 🎯 Modes debug vs release

### Mode Debug (par défaut)

- ✅ Compilation **rapide**
- ✅ Symboles de debug inclus
- ❌ **Lent à l'exécution** (10-100x plus lent)
- 💾 Binaire plus volumineux

```bash
cargo build
cargo run
cargo test
```

### Mode Release

- ❌ Compilation **lente** (optimisations)
- ✅ Exécution **ultra-rapide**
- ❌ Pas de symboles debug
- 💾 Binaire plus petit

```bash
cargo build --release
cargo run --release
```

**⚠️ Règle d'or** : Développez en debug, livrez en release !

## 🔍 Dépannage Rapide

### "Command not found: cargo"

```bash
# Installer Rust/Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Erreur de compilation mystérieuse

```bash
cargo clean    # Nettoie le cache
cargo build    # Recompile tout
```

### Binaire trop volumineux

```bash
# Voir ce qui prend de la place
cargo install cargo-bloat
cargo bloat --release

# Optimiser dans Cargo.toml
[profile.release]
opt-level = "z"     # Optimise la taille
lto = true          # Link-Time Optimization
strip = true        # Retire les symboles
```

### Tests qui échouent aléatoirement

```bash
# Exécuter les tests en séquentiel
cargo test -- --test-threads=1
```

## 📚 Ressources officielles

Pour aller plus loin :

- **[The Cargo Book](https://doc.rust-lang.org/cargo/)** - Documentation complète
- **[The Rust Book](https://doc.rust-lang.org/book/)** - Apprendre Rust
- **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)** - Exemples pratiques
- **[crates.io](https://crates.io/)** - Registre des bibliothèques
- **[docs.rs](https://docs.rs/)** - Documentation des crates
- **[Rust Playground](https://play.rust-lang.org/)** - Tester du code en ligne

## 🎓 Conseils de pro

### ✅ À faire

- Lancer `cargo clippy` avant chaque commit
- Utiliser `cargo check` pendant le développement (plus rapide)
- Commiter `Cargo.lock` pour les binaires
- Utiliser `--release` pour les benchmarks/démos

### ❌ À éviter

- Ne jamais commiter le dossier `/target`
- Ne pas ignorer les warnings de clippy
- Ne pas tester les performances en mode debug
- Ne pas modifier `Cargo.lock` manuellement

## 🚦 Démarrage rapide - nouveau projet

```bash
# Créer un nouveau projet binaire
cargo new mon-app
cd mon-app

# Ajouter une dépendance
cargo add serde

# Développer
cargo check        # Vérifier
cargo run          # Tester

# Avant de commit
cargo fmt          # Formater
cargo clippy       # Analyser
cargo test         # Tests

# Release finale
cargo build --release
./target/release/mon-app
```

---

**🎯 Résumé** : Les 5 commandes essentielles à connaître :

1. `cargo check` - Vérifier rapidement
2. `cargo clippy` - Analyser la qualité
3. `cargo test` - Lancer les tests
4. `cargo run` - Exécuter
5. `cargo build --release` - Build final

Pour tout le reste → [Documentation officielle Cargo](https://doc.rust-lang.org/cargo/)
