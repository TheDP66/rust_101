Resume [here](https://youtu.be/lzKeecy4OmQ?t=17936)

## Create application

`cargo init .`

## Run application

`cargo run`

## Run a file

### With compile detail

`cargo run --bin a1`

## Without compile detail

`cargo run -q --bin a1`

## Generate documentation
- add `/// documentation here` above all that needs to be documented 
- then run `cargo doc --open`
    - not open? using WSL? 
    - add `export BROWSER="/mnt/c/Program Files (x86)/Microsoft/Edge/Application/msedge.exe"` to `.bashrc` or `.zshrc`
    - then restart terminal

## Type

### enum
used for listing dropdown option.

### struct
used for telling object detail.

### impl
used for implement some function to struct.

### vec
same like array

### match
same like switch, but more strict