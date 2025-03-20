# RustyDex Library

This is intended to be the Rustydex main library, separating much of the dex
logic from the frontend application and hopefully allowing this crate to be
useful to other future applications that may wish to utilize it.

## Purpose

This library has a couple of purposes at present, which are important to note
for any application intending to use it:

1. This is not just a code repository. The library is intended to be a
   statically available representation of the data available at present. What
   that means is the data in this library will be loaded at runtime whether you
   use it or not. Right now, an effort is made to ensure the data usage is not
   excessive, but no guarantees are made about the exact amount of memory
   loading this library will use.
2. Although the library contains a significant amount of static data, it will
   (eventually) support pulling more recent data down at runtime, if the
   application requests that it do so. Initial static values are immutable, of
   course, but effort will be taken to overwrite the static data with no data
   where appropriate rather than loading more than necessary into memory. The
   library will also support loading data from a file on disk, and in fact this
   will be encouraged along with the previous method to help ensure unnecessary
   data fetches are not made and to allow your application to have the most
   up-to-date information.

The primary benefit of this approach is that the frontend application doesn't
have to start by loading a ton of data from an API or file, and data is
available even on a first run in the event that no network is available to pull
down files or no filesystem access is available for some reason to saves data
externally to the application itself. We also maintain independence from
external data sources such as pokeapi, while still allowing such APIs to be used
for fetching data at runtime.
