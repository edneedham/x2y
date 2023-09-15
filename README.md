# x2y

Transcode data-serialization files from x to y.

## How it works

You have some file(s) in a particular data-serialization language and you want 
them in a different language. 


#### Converting all files that match the input format in a directory

```bash
> x2y -x yaml -y json .
```
Any files that are in the current working directory that have a yaml format are 
converted to json format.

#### Converting a single file

```bash
> x2y -y yaml config.json
```
When a single file is specified, the option for the input format is no longer 
necessary as it is taken from the input file.


## Supported file formats 

* Yaml 
* Json
* Toml 


## Installation

#### From source if you have rust and cargo installed

```bash
> cargo install --locked x2y
```
