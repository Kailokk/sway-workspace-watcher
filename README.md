
# Sway Workspace Watcher

A miniscule app that will listen for sway events, and output the current state of workspaces at the time of the event.

##### Sample Output

```json
[
    {
        "name": "1",
        "output": "DVI-D-1",
        "visible": false,
        "urgent": false,
        "focused": false
    },
    {
        "name": "2",
        "output": "DVI-D-1",
        "visible": true,
        "urgent": false,
        "focused": true
    },
    {
        "name": "3",
        "output": "HDMI-A-1",
        "visible": true,
        "urgent": true,
        "focused": false
    }
]
```

## Installation

### Nix Flake

You can include the project as part of a flake like so:

```nix
{
 inputs = {
  nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";

  home-manager = {
   url = "github:nix-community/home-manager/release-23.11";
   inputs.nixpkgs.follows = "nixpkgs";
  };

  sway-workspace-watcher.url = "github:kailokk/sway-workspace-watcher";
 };
}
```

Then simply add this line to your configuration:

```nix
environment.systemPackages = [ inputs.sway-workspace-watcher.packages.${pkgs.system}.default ];
```

### Crates/Cargo

You can build/install this package directly using cargo:

```sh
cargo install --git "https://github.com/Kailokk/sway-workspace-watcher"
```

## Contributions

This is intended as a very small app to help me bridge the gap between sway and eww, to display the workspace state.
That being said, if you have any suggestions for improvements or features, I would love to hear them! :)
