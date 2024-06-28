# rust utils

## hex
print the decimal number string to hexadecimal string.

### Uasge
 
```shell
cargo build --package hex # or --release

./target/debug/hex dec_str0 dec_str1 .. dec_strn

# output: hex_str0 hex_str1 .. hex_strn
```

## strop
reverse hex_str endian format

### Uasge
 
```shell
cargo build --package strop # or --release

./target/debug/strop rev hex_str -n num_bytes

#output: rev_hex_str
```

## gitext
gitc => git clone

### Uasge
 
```shell
cargo build --package gitc # or --release

./target/debug/gitc user/repo:(reename)/branch(main)

#example: gitc zilong-dai/rust-utils
```

