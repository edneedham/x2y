# x2y

Transcode data-serialization files from x to y.

## How it works

Say you have some file(s) in a particular data-serialization language,
we can call this x, and you want them in a different language, we can call this
y: you see where we're going here? Well by running this:

```bash
> x2y -x toml -y yaml .
```
We transcode any files that are in the current working directory that have a toml
format into a yaml format.

## More examples

Converting a single file

```bash
> x2y -y yaml config.json
```
When a single file is specified, the option for the input format is no longer 
necessary as it can be taken from the input file.

## Supported file formats 

* Yaml 
* Json
* Toml 

## Installation

### From source

```bash
cargo install --locked x2y
```

### Package Managers

On Ubuntu

```bash
> sudo apt install x2y
```

On Mac

```bash
> brew install x2y
```

On Windows 

```bash
> choco install x2y
```






