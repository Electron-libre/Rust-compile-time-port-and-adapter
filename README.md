# Rust compile time port and adapter architecture

This showcase a simple way to define multiple implementations for a single port without involving dynamic dispatch.

## Use case

Let say you're building an service, that depends on a database:
* You probably need an in memory implementation of the database for unit testing.
* You may want your service to depend on postgres implementation by default but provide another binary to use with SqlLite.

## How

The provided code show a way to achieve this by using conditional compilation and feature flags.
The port implementations does nothing more thant setting a boolean and logging.


Building and running with DefaultAdapter : 

```
cargo run
```

```
[2023-08-28T13:57:17Z DEBUG rust_compile_time_adapter] Default Adapter done something
```

Building and running with OtherAdapter :

```
cargo run --features other_adapter
```

Running tests:

```
[2023-08-28T14:01:07Z DEBUG rust_compile_time_adapter] Default Adapter done something
[2023-08-28T14:01:07Z DEBUG rust_compile_time_adapter] Other Adapter done something
[2023-08-28T14:01:07Z DEBUG rust_compile_time_adapter::test] Test Adapter done something
```

## Pros

This allows to use static dispatch to optimize performances and binary size.

It's highly maintainable and simple.

## Cons

Mutually exclusives features can be tedious to maintain.

Not the solution if you need to switch adapter at runtime

## Remarks

I placed all the code and tests in the main module for convenience.
In real application you'll probably want to split modules. 
