rust-jimraft
============
rust wrapper for JimRaft(https://github.com/jimdb-org/jimraft) written by C++.


## Requirements

- Clang and LLVM

## Contributing

Feedback and pull requests welcome!  If a particular feature of Jimraft is 
important to you, please let me know by opening an issue, and I'll 
prioritize it.

## Usage

This binding is statically linked with a specific version of Jimraft. If you 
want to build it yourself, make sure you've also cloned the Jimraft and 
compression submodules:

    git submodule update --init --recursive
