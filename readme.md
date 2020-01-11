# blake3-native

**Node.js bindings to the [BLAKE3 hash function](https://github.com/BLAKE3-team/BLAKE3).**

⚠️ This lib is a work-in-progress, check out [the Issues page](https://github.com/derhuerst/blake3-native/issues) for things to be done. I'm a Rust newbie, so expect non-idiomatic code!

[![npm version](https://img.shields.io/npm/v/blake3-native.svg)](https://www.npmjs.com/package/blake3-native)
[![build status](https://api.travis-ci.org/derhuerst/blake3-native.svg?branch=master)](https://travis-ci.org/derhuerst/blake3-native)
![ISC-licensed](https://img.shields.io/github/license/derhuerst/blake3-native.svg)
![minimum Node.js version](https://img.shields.io/node/v/blake3-native.svg)
[![chat with me on Gitter](https://img.shields.io/badge/chat%20with%20me-on%20gitter-512e92.svg)](https://gitter.im/derhuerst)
[![support me on Patreon](https://img.shields.io/badge/support%20me-on%20patreon-fa7664.svg)](https://patreon.com/derhuerst)

> BLAKE3 is a cryptographic hash function that is:
>
> - **Much faster** than MD5, SHA-1, SHA-2, SHA-3, and BLAKE2.
> - **Secure**, unlike MD5 and SHA-1. And secure against length extension,
  unlike SHA-2.
> - **Highly parallelizable** across any number of threads and SIMD lanes,
  because it's a Merkle tree on the inside.
> - Capable of **verified streaming** and **incremental updates**, again
  because it's a Merkle tree.
> - A **PRF**, **MAC**, **KDF**, and **XOF**, as well as a regular hash.
> - **One algorithm with no variants**, which is fast on x86-64 and also
  on smaller architectures.

> *NOTE: BLAKE3 is not a password hashing algorithm, because it's
designed to be fast, whereas password hashing should not be fast. If you
hash passwords to store the hashes or if you derive keys from passwords,
we recommend [Argon2](https://github.com/P-H-C/phc-winner-argon2).*


## Installation

```shell
npm install derhuerst/blake3-native
```


## Usage

```js
todo
```


## Contributing

If you have a question or need support using `blake3-native`, please double-check your code and setup first. If you think you have found a bug or want to propose a feature, refer to [the issues page](https://github.com/derhuerst/blake3-native/issues).
