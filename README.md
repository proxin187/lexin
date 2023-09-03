# Lexin

A portable lexer.

## Description

Lexin is a portable lexer that can easily be packaged with any application and has is able to lex all kinds of syntax's.

## Getting Started

### Dependencies

These Dependencies are only required when building.
* rustc
* cargo
* python

### Installing

* Clone the repository
```
git clone https://github.com/proxin187/lexin
```
* Run the build script
```
python build.py
```

### Executing program

* Create a lex file (see lex section)
* The program is executed the way shown below
```
lexin [lex file] [file to lex] [options]
```

## Help

Currently there is no bound checking on the indexing which means that unfinished statements in the lex file may cause a panic.

## Authors

Contributors names and contact info

* [Proxin](https://github.com/proxin187)

## Version History

* 0.2
    * README.md updates
    * See [commit change]() or See [release history]()
* 0.1
    * Initial Release

## License

Currently there is no license, this may change in the future


