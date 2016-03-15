# Rust IPLD

[![Build Status](https://travis-ci.org/dignifiedquire/rust-ipld.svg?branch=master&style=flat-square)](https://travis-ci.org/dignifiedquire/rust-ipld) [![Clippy Linting Result](https://clippy.bashy.io/github/dignifiedquire/rust-ipld/master/emojibadge.svg?style=flat-square)](https://clippy.bashy.io/github/dignifiedquire/rust-ipld/master/log)

> Experiments in implementing types around [IPLD] for typesafe gurantees at compile time.

Main goal: Find a parser combinator like structure to define 1:1 invertible mappings from
different IPLD objects to arbitrary graph structures.

[IPLD]: https://github.com/ipfs/specs/blob/master/merkledag/ipld.md
