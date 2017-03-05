grnenv-rs
===

[![Build Status](https://travis-ci.org/cosmo0920/grnenv-rs.svg?branch=master)](https://travis-ci.org/cosmo0920/grnenv-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/2m8wmd96h4k7f2om/branch/master?svg=true)](https://ci.appveyor.com/project/cosmo0920/grnenv-rs/branch/master)

grnenv-rs is a tool to switch using multiple Groonga versions.

Currently, only confirmed to work on Windows 10, macOS Sierra(10.12), and Ubuntu Trusty (14.04 LTS).

## Prerequisites

### Windows + just download executables

* Visual C++ Redistributable for Visual Studio 2015

x86_64 msvc target binaries are found at:
https://github.com/cosmo0920/grnenv-rs/releases

And then, put binaries into `$Env:USERPROFILE\bin`.

### Windows + build yourself

* rustup

### *nix via cargo install

Prepare the following dependent libraries:

* C and C++ compilers like gcc or clang
* autoconf
* automake
* libtool
* pkg-config
* pcre

#### download executables

##### x86\_64 Linux

x86_64 musl Linux binaries are found at:
https://github.com/cosmo0920/grnenv-rs/releases

* grnenv-rs-VERSION-x86_64-unknown-linux-musl.tar.xz

And then, put binaries into `$HOME\bin`.

##### x86\_64 macOS

x86_64 macOS binaries are found at:
https://github.com/cosmo0920/grnenv-rs/releases

* grnenv-rs-VERSION-x86_64-apple-darwin.zip

And then, put binaries into `$HOME\bin`.

#### build grnenv yourself

Prepare the following toolchain:

* rust compiler (installed with rustup)

And then,

```bash
$ cargo install grnenv-rs
```

## Plugin system

This tool has plugin system, which is using subcommand mechanism.

You should put `grnenv-*` executables into `$PATH` or `~/bin`.

### Notes

In Windows, `grnenv-*` subcommand should be portable executables.
If you want to support Windows in subcommand, it recommends to use Rust language and build (i686|x86_64)-pc-windows-msvc target.

## Usage

### For Windows


```powershell
PS> grnenv init
```

And then,
Please create profile.ps1 the following place:

```powershell
$Env:USERPROFILE\Documents\WindowsPowerShell\profile.ps1
```

And write the following thing:

```powershell
. $Env:USERPROFILE\.groonga\shims\bin\source-groonga.ps1
```

then,

```powershell
PS> grnenv install VERSION [--arch (x86|x64)]
PS> grnenv switch VERSION [--arch (x86|x64)]
```

Finally, restart powershell and use specified version of Groonga.

If you get an error, you should specify execution policy as follows:

```powershell
PS> Set-ExecutionPolicy RemoteSigned
```

### For *nix environment

```bash
$ grnenv init
```

And write the following content into .bash\_profile or .zsh\_profile etc.:

```bash
. $HOME/.groonga/shims/bin/source-groonga.sh
```

then,

```bash
$ grnenv install VERSION
$ grnenv switch VERSION
```

Finally, restart your shell and use specified version of Groonga.

## LICENSE

[MIT](LICENSE).

## Related Articles

* 複数の環境で動作するバージョン切り替えツールのgrnenv-rsを作ってみたお話
http://qiita.com/cosmo0920/items/0a975fb4509114e9b189
