# `calculate-contacts`

This is a simple program to calculate the contacts in a single PDB file with multiple chains written in [Rust](https://www.rust-lang.org/tools/install).

## Installation

Compile it with [cargo](https://www.rust-lang.org/tools/install)

```bash
$ cargo build --release
    Finished release [optimized] target(s) in 0.00s
```

Copy the binary to wherever you need it

```bash
$ cp target/release/calculate-contacts $HOME/bin # or anywhere else
```

## Usage

```bash
calculate-contacts <input.pdb> <cutoff>
```

Example:

Download some PDB and calculate all the contacts in it

```bash
$ wget https://files.rcsb.org/view/2OOB.pdb
$ ./calculate-contacts 2OOB.pdb 5.0
GLN N B 49 PHE CG A 946 4.702
GLN N B 49 PHE CD1 A 946 4.434
GLN N B 49 PHE CD2 A 946 4.496
GLN N B 49 PHE CE1 A 946 3.915
GLN N B 49 PHE CE2 A 946 3.986
GLN N B 49 PHE CZ A 946 3.662
GLN CA B 49 PHE CG A 946 4.785
GLN CA B 49 PHE CD1 A 946 4.636
GLN CA B 49 PHE CD2 A 946 4.836
GLN CA B 49 PHE CE1 A 946 4.52
# ...
```

* * *

