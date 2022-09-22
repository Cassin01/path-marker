# path-marker

## Installation

```sh
cargo install path-maker
```

<details>
<summary>Uninstall</summary>

1. uninstall `path-marker`

```zsh
cargo uninstall path-marker
```

2. remove configuration file will be automatically generated on:

- Linux: `~/.config/rcz`
- Windows: `{FOLDERID_RoamingAppData}\rcz`
- Mac OS: `~/Library/Preferences/rs.rcz`

3. remove `~/.cache/path_marker/hist.txt`

</details>


## Usage

```sh
path-marker -- mark # marks current path
path-marker -- show # shows all paths that were marked
path-marker -- conf # shows a configuration info.
```

## Example

```zsh
# Changing directory using fuzzy find.
function cdm {
  cd `path-marker -- show | peco`
}
alias mp='path-marker -- mark'
# alias ms='path-marker -- show'
```
