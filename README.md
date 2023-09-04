[![Stars](https://img.shields.io/github/stars/proxin187/lexin.svg?style=for-the-badge)](https://github.com/proxin187/lexin/stargazers)
[![Forks](https://img.shields.io/github/forks/proxin187/lexin.svg?style=for-the-badge)](https://github.com/proxin187/lexin/forks)

# Lexin

A portable lexer.

<li>
    <a href="#Description">Description</a>
</li>
<li>
    <a href="#getting-started">Getting Started</a>
    <ul>
    <li><a href="#Dependencies">Dependencies</a></li>
    <li><a href="#Installing">Installing</a></li>
    <li><a href="#Usage">Usage</a></li>
    </ul>
</li>
<li>
    <a href="#Lex">Lex File</a>
    <ul>
    <li><a href="#Keywords">Keywords</a></li>
    <li><a href="#Symbols">Symbols</a></li>
    <li><a href="#Sections">Sections</a></li>
    <li><a href="#Name">Name</a></li>
    </ul>
</li>
<li><a href="#Help">Help</a></li>
<li><a href="#Authors">Authors</a></li>
<li><a href="#Versions">Versions</a></li>
<li><a href="#License">License</a></li>

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

### Usage

* Create a lex file [see lex section](#lex)
* The program is executed the way shown below
```
Usage: lexin [config(*.lex)] [target] [options]
  Options:
    -format: [formats] defaults to python
      formats:
        json: output tokens in json format
        python: output tokens in python format
```

## Lex
Lexin uses a .lex file to input keywords and symbols to the lexer

### Keywords
Syntax: `_keywords [value]*`
Example:
```
_keywords "fn" "if" "else" "match" "use"
```
### Symbols
Syntax: `_symbols [value]*`
Example:
```
_symbols "+" "-" "*" "/" "{" "}" "[" "]" "(" ")" "<" ">" ";" ":"
```
### Sections
Syntax: `_sections [[start] - [end] [name]]*`
Example:
```
_sections
    "/*" - "*/" "comment"
```
### Name
Names only apply to symbols
Syntax: `_name [[value] [name]]*`
Example:
```
_name
    "+" "Plus"
    "-" "Minus"
```

## Help

Currently there is no bound checking on the indexing which means that unfinished statements in the lex file may cause a panic.

## Authors

Contributors names and contact info

* [Proxin](https://github.com/proxin187)

## Versions

* 0.2
    * README.md updates
    * See [commit change]() or See [release history]()
* 0.1
    * Initial Release

## License

Currently there is no license, this may change in the future


