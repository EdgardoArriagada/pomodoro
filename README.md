this is a WIP

to develop
```zsh
rm /tmp/rust-uds.sock; cargo run
```

to run, create alias
```zsh
pmdr() {
  print "$@" | socat UNIX-CONNECT:/tmp/rust-uds.sock -
}
```
and run like
```zsh
pmdr 10m doing stuff
```

dependencies:
 - socat
