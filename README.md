# Rust Robot Resource Collection Simulation

Ce projet est une simulation de robots collectant des ressources dans un environnement généré procéduralement. Les robots naviguent de manière autonome sur une carte, collectent des ressources et les ramènent à une base centrale.

## Fonctionnalités

- **Robots multi-threadés** : Chaque robot fonctionne dans son propre thread
- **Différentes classes de robots** : 
  - Scientifiques (collectent la recherche)
  - Mineurs (collectent le fer)
- **Navigation autonome** : Les robots utilisent un algorithme A* pour trouver les chemins optimaux
- **Adaptation au terrain** : Les robots s'équipent automatiquement en fonction du terrain environnant:
  - Bouées pour l'eau
  - Chenilles pour les montagnes
  - Roues pour les terrains normaux
- **Carte procédurale** : Générée avec l'algorithme de Perlin noise
- **Base centrale** : Pour recharger les robots et stocker les ressources collectées
- **Système d'énergie** : Les robots doivent gérer leur énergie et retourner à la base pour se recharger
- **Interface utilisateur terminal** : Affichage temps réel avec ratatui

## Installation

### Prérequis

- Rust (version stable récente)
- Un terminal compatible avec les séquences d'échappement ANSI

### Étapes d'installation

1. Clonez le dépôt:
   ```bash
   git clone https://github.com/votre-username/rust-robot-simulation.git
   cd rust-robot-simulation
   ```

2. Compilez le projet:
   ```bash
   cargo build --release
   ```

## Lancement

Pour lancer la simulation avec les paramètres par défaut:

```bash
cargo run --release
```

## Configuration

Les principaux paramètres se trouvent en haut de la fonction main() dans `src/main.rs`:

```rust
const WIDTH: usize = 100;
const HEIGHT: usize = 50;
const NUM_ROBOTS: usize = 10;
const SEED: u32 = 42;
```

Pour modifier ces paramètres, changez les valeurs dans le code et recompilez le projet.

### Modifier le nombre de robots

Changez la constante `NUM_ROBOTS` pour augmenter ou diminuer le nombre de robots dans la simulation.

### Modifier la taille de la carte

Ajustez les constantes `WIDTH` et `HEIGHT` pour changer les dimensions de la carte.

### Modifier la seed

Changez la valeur de `SEED` pour générer une carte différente. Chaque valeur de seed produira un monde unique mais reproductible.

## Contrôles

- **q** : Quitter la simulation
- **u** : Améliorer la base (augmente la capacité d'énergie et de stockage)

## Fonctionnement des robots

- Les robots alternent entre collecter des ressources et retourner à la base
- Ils retournent à la base automatiquement quand leur niveau d'énergie est bas
- À la base, ils déposent leurs ressources, se rechargent, et peuvent changer d'équipement
- Les scientifiques collectent de la recherche, les mineurs collectent du fer

## Développement et extension

Pour ajouter de nouvelles fonctionnalités ou modifier le comportement:

- **Nouveaux types de robots** : Modifiez la classe dans la création des robots dans `robot.rs`
- **Nouveaux équipements** : Ajoutez des options dans `modify_robot_equipment` dans `base.rs`
- **Nouveaux biomes** : Modifiez les fonctions de génération dans `map.rs`

## Structure du projet

- **main.rs** : Point d'entrée et boucle principale
- **robot.rs** : Logique des robots
- **map.rs** : Génération et gestion de la carte
- **base.rs** : Logique de la base centrale