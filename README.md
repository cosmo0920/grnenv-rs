grnenv-rs
===

grnenv-rs is a tool to switch using multiple Groonga versions.

Currently, only supported for Windows.

## Prerequisites

### Windows + just download executables

* Perhaps, Visual C++ Redistributable for Visual Studio 2015 is needed

Binary upload plan is TBD.

### Windows + build yourself

* rustup
* [OpenSSL for MSVC](https://slproweb.com/products/Win32OpenSSL.html)

And follow the descriptions:

https://github.com/sfackler/rust-openssl#windows

### *nix

* openssl development package (libssl-dev or openssl-devel like package)
* rust compiler (installed with rustup)

#### Note

If you installed openssl library in non-standard place, please specify `OPENSSL_PKG_CONFIG_PATH` environment variable like this:

```bash
$ export OPENSSL_PKG_CONFIG_PATH=/path/to/installed/openssl
```

## Usage

### For Windows

git clone and,

```powershell
PS> cargo install
PS> grnenv-rs init
```

And then,
Please create profile.ps1 the following place:

```
$Env:USERPROFILE\Documents\WindowsPowerShell\profile.ps1
```

And write the following thing:

```
. $Env:USERPROFILE\.groonga\shims\bin\source-groonga.ps1
```

then,

```
PS> grnenv-rs install VERSION [--arch (x86|x64)]
PS> grnenv-rs switch VERSION [--arch (x86|x64)]
```

Finally, restart powershell and use specified version of Groonga.

### For *nix environment

git clone and,

```bash
$ cargo install
$ grnenv-rs init
```

And write the following into .bash\_profile or .zsh\_profile etc.:

```
. $HOME/.groonga/shims/bin/source-groonga.sh
```

then,

```
$ grnenv-rs install VERSION
$ grnenv-rs switch VERSION
```

Finally, restart your shell and use specified version of Groonga.

## LICENSE

[MIT](LICENSE).
