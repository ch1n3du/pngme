# pngme 🖼️

A simple  to use CLI for  encoding secret messages inside of images.

## Description 

A simple  to use CLI for  encoding secret messages inside of images.
This is an implementation of the [pngme](https://picklenerd.github.io/pngme_book/) rust book.


## Getting Started

### Dependecies
- Install the [rustup](https://rustup.rs/) toolchain.


### Installing
- Simply run `cargo install pngme`.


## Usage 


### Encoding Messages

The `encode` command takes three compulsory arguments:

- The file path of the input file.
- The chunk type.
- The message to be stored.

```
$ pngme encode ./dice.png ruSt "This is a secret message!"
```

Note: It can take an optional argument, a file path for the ouput file to avoid overwriting the input file.
```
$ pngme encode input_file chunk_type message output_file
```


### Decoding messages
```
$ pngme decode input_file chunk_type
```

### Removing messages
```
pngme remove file_path chunk_type
```

### Printing messages
```
pngme print file_path
```