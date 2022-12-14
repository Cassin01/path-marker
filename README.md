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

- Linux: `~/.config/path-marker`
- Windows: `{FOLDERID_RoamingAppData}\path-marker`
- Mac OS: `~/Library/Preferences/rs.path-marker`

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
function cdm() {
  local marker_list output
  if marker_list=$(path-marker -- show); then
    if output=$(echo "${marker_list}" | peco); then
      cd "${output}" || exit
    fi
  fi
}
alias mp='path-marker -- mark'
# alias ms='path-marker -- show'
```
