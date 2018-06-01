# ohyo-tokyo: Rust tokyocabinet bindings

This is fork of [tokyocabinet-sys](https://github.com/ehiggs/tokyocabinet-sys.git) crate.  

Note: atm it in development state. So interface can changed. 

## Why i made this crate

I just need tokyocabinet bindnings for my purposes which used system wide installed tokyocabinet library.
This crate will used in [asap](https://github.com/ohyo-io/asap) database. 

Almost-complete bindings for leveldb for Rust.

fork of Tokyocabinet-sys

A crate for accessing low level functions in the TokyoCabinet key value store.

- [x] Abstract DB (`tcadb`)
- [x] B+ Tree DB (`tcbdb`)
- [x] Hash Table DB (`tchdb`)
- [ ] Fixed length DB (`tcfdb`)
- [ ] Table DB (tctdb)

- [ ] Cursors (`tccur`)
- [x] Extensible Strings (`tcxstr`)
- [x] Array List (`tclist`)
- [ ] Ordered Tree (`tctree`)
- [ ] In Memory B+ Tree DB (`tcndb`)
- [ ] In Memory Hash Table DB (`tcmdb`)
- [ ] Memory Pool

## License

All the code in this repository is released under the ***BSD 2-Clause License***, for more information take a look at the [LICENSE] file.
