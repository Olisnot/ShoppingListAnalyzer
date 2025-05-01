# ShoppingListAnalyzer

## Building the application
This repository is set up for building the application in a Nix shell.

1. Install the Nix package manager: https://nixos.org/
2. Enable flake support by creating a file in either  ~/.config/nix/nix.conf or /etc/nix/nix.conf and writing the following to the file: 
```
experimental-features = nix-command flakes
```
3. Run the following command in a terminal ``` nix develop ```.
4. Once the nix shell has been created run the following command inside the shell ```cargo run --release```
