# External memory tools

This is a tiny collections of tools useful to address memory that is not mapped directly into RAM. Do not use it unless you know exactly what you are doing, the cases where this is needed are very limited and mostly related to very special baremetal systems.

## Brief explanation

Rust allows neat memory allocations on no-std systems with the use of allocator abstraction; however, all normal tools assume that the memory that is addressed by that allocator could be low-level mapped into some address space. In some rare cases this is not possible (for example, in security vaults mapping arbitrary memory would present a direct breach to system security model).

So if you happen to work with one of those systems and you find yourself repeating the same patterns over and over again - use this crate.

## Usage

Implement needed buffer access operations on your target memory and enjoy!

To use this crate with regular memory (for simpler cross platformness), just use `()` as `External Memory` - feature is implemented there. Unfortunately, you would still have to include `()` as parameter in every affected function call.

## Development

Currently only read operations are supported; if you decide to contribute and add more features like writable and read-writeable buffers, please start hiding those under feature flags to keep things lean and safe.

