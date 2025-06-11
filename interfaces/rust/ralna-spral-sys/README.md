# RALNA SPRAL Rust Bindings

This crate contains Rust bindings to the C interface of
[SPRAL: The Sparse Parallel Robust Algorithm Library](https://github.com/ralna/spral "Spral Repository") -
an open-source (BSD) library for sparse linear algebra and associated algorithms.

## Packages

- **LSMR** - Solves sparse least squares problems using LSMR
  algorithm.
- **RANDOM** - Pseudo-random number generator.
- **RANDOM_MATRIX** - Generates random matrices for testing purposes.
- **RUTHERFORD_BOEING** - Read and write matrices in Rutherford-Boeing
  format.
- **SCALING** - Calculates matrix scalings through a variety of
  algorithms
- **SSIDS** - Sparse Symmetric Indefinite Direct Solver.
- **SSMFE** - Sparse Symmetric Matrix-Free Eigensolver. Uses
                      Jacobi-conjugate preconditioned gradients
                      method.

## Installation

This crate **does not** install SPRAL for you, it requires a pre-existing
installation to be available.

For information on how to install SPRAL, please refer to the
[README](https://github.com/ralna/spral?tab=readme-ov-file#installation "SPRAL README installation section")
or the
[C interface installation documentation](https://ralna.github.io/spral/_build/html/C/install.html "C interface installation documentation").

### Build Options

The `meson` build of SPRAL supports options for configuring the build, notably with different libraries for BLAS, LAPACK, METIS, OpenMP, and hwloc.
This crate supports similar configuration through the use of environment variables.
The table below documents each of the environment variables that can be used to configure the build of this crate:

| Name | Corresponding `meson` option | Default Value |
| ---- | ---------------------------- | ------------- |
| `SPARAL_LIBBLAS` | `libblas` | `blas` |
| `SPARAL_OPENMP` | Inferred if `openmp` option set | `gomp` |
| `SPARAL_LIBLAPACK` | `liblapack` | `lapack` |
| `SPARAL_LIBHWLOC` | `libhwloc` | `hwloc` |
| `SPARAL_LIBMETIS` | `libmetis` | `metis` |

These options are used to inform `rustc` which libraries must to link against
when building your project.

If the libraries you would typically use for these options require some special
setup, then this must be provided as custom configuration in your project.
