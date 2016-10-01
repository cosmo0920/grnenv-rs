grnenv-rs
===

grnenv-rs is a tool to switch using multiple Groonga versions.

Currently, only supported for Windows.

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

Not supported, but patches are welcome!

## LICENSE

[MIT](LICENSE).
