# Workshop Rust

Petit projet rust fait pour appréhender les bases du language

Crée par `NoeBaiocchi` and `AmozPay`.

[![forthebadge](https://forthebadge.com/images/badges/built-with-love.svg)](https://forthebadge.com)

## Avant de commencer

L'objectif de ce workshop est d'aprendre les bases de Rust en suivant une certaine ligne directive pour ne pas etre perdu,
mais libre de modifié la thematique du projet de base pour faire autre chose qu'un programme de calcule mental :)

## Technologies
Le projet utilise:
* Rust
* Cargo
* Crates.io
	
## Setup

### Prerequis
* Install Rust 
	*  Run cet commande -> ```$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
	*  Site officiel -> `https://www.rust-lang.org/tools/install`

### Utils
* Documentation RustBook -> `https://doc.rust-lang.org/book/`
* Library/Package -> `https://crates.io/`


### Lancée le projet Exemple
```
$ open a terminal
$ clone the repository in your computer`
$ cd WORKSHOPRUST
$ cargo build
$ cargo run [CMD]
& cargo clippy
```
## Workshop I: Hello World
1) Utiliser Cargo pour initialiser votre projet
*	```$ cargo new [NAME] ```

2) Normalement vous aurez :
*	Un dossier src contenant un `main.rs`
*	Un dossier `target` utilisé pour generer et utiliser la documentation
*	Un fichier `Cargo.lock` pour utilisé pour les packages
*	Un fichier `Cargo.toml` que vous devez utiliser pour definir vos package/libs

3) Lancer le projet
* 	```$ cargo build``` -> Build votre projet pour voir si il compile
*	```$ cargo run [ARGS]``` -> Build et lance votre projet avec les possible arguments
*	```$ cargo clippy``` -> Verifier que votre programme est `Safe` celon les critéres de clippy


## Workshop II: Build IT
