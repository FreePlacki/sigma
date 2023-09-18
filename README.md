# Installation

```bash
git clone --depth=1 https://github.com/FreePlacki/sigma.git &&
cd sigma &&
cargo build --release 
```

You can put the execuatble in `~/.local/bin`:
```bash
cp target/release/sigma ~/.local/bin
```

Add to path (if you don't already have it):
```bash
export PATH=$PATH:~/.local/bin
```

Create `~/.sigma` and put in `constants.txt`:
```bash
mkdir ~/.sigma && cp constants.txt ~/.sigma
```

# Usage
Run:
```bash
sigma
```
without arguments to enter repl or provide a file as an argument.

For example usage see example.txt
