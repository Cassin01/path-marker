# path-marker

## Installation

```sh
cargo install path-maker
```

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
