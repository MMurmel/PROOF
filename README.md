# Learning Partial Boolean Functions by Local Search
PROOF (for **P**a**R**tial b**OO**lean **F**unctions) is my implementation for my research project
at the chair of [Machine Learning for Computer Vision](https://mlcv.inf.tu-dresden.de/)
at [Technische Universität Dresden](https://tu-dresden.de/).

## Dependencies
* `conda` for preparing the data
* `rust` to compile PROOF

## Usage
### DATA
> ***NOTE:*** The data format will change soon and might change again at any time if that 
> makes accessing the data easier. This, combined with the wish to keep this repository small,
> lead to the decision that only the raw data and a conversion script will be provided.
> To run PROOF the data must be recreated first!
>
>For details on the chosen data format and how to recreate the data see [here](data/Data.md).

### Compilation
Simply run `cargo build --release` to compile PROOF.
The `--release` here makes for a huge improvement in runtime that is well worth the extra seconds in compile time.
Optionally, you would want to symlink the resulting binary for easy access by `ln -s target/release/proof proof`

### Running PROOF
PROOF can be configured via `proof -c [config_file]`.
For testing purposes the `testing_config.json` is provided.
> ***NOTE:*** It is _highly_ recommended to only run `proof` with the testing configuration for now,
> as its runtime is still to slow to tackle real world circumstances.

Verbosity of debug information can be increased by each additional `-v`.

## Licensing
This project is licensed under the [GNU General Public License v3](https://www.gnu.org/licenses/gpl-3.0.txt) except for those parts (lines of code from libraries used in this project) already licensed under other licenses.

## Copyright
This projects structure is based on a [custom](https://github.com/MMurmel/themis) version of
[Georg Lauterbach](https://github.com/georglauterbach)'s [themis](https://github.com/georglauterbach/themis)
template for command line rust programs.
