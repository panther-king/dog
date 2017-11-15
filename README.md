# dogrun

A joke command that is inspired by `cat(UNIX)`.

## What's this?

`cat` displays file contents on stdout.

However, `dog` command eats file contents.<br>
This means file will be empty when `dog` command executed.

This is not usefull at all.<br>
So what? `dog` is just a joke command :)

## Usage

    $ ls -l
    total 12
    drwxr-xr-x 2 taro taro  35 Nov 15 08:37 src
    drwxr-xr-x 3 taro taro  19 Nov 15 07:46 target
    drwxr-xr-x 2 taro taro  20 Nov 14 08:23 tests
    -rw-r--r-- 1 taro taro 412 Nov 14 08:22 Cargo.lock
    -rw-r--r-- 1 taro taro 185 Nov 15 07:45 Cargo.toml
    -rw-r--r-- 1 taro taro 310 Nov 16 08:33 README.md

    $ echo bow-wow > dog.txt
    $ cat dog.txt
    bow-wow

    $ cargo build
    $ ./target/debug/dog dog.txt

    $ ls -l
    total 12
    drwxr-xr-x 2 taro taro  35 Nov 15 08:37 src
    drwxr-xr-x 3 taro taro  19 Nov 15 07:46 target
    drwxr-xr-x 2 taro taro  20 Nov 14 08:23 tests
    -rw-r--r-- 1 taro taro 412 Nov 14 08:22 Cargo.lock
    -rw-r--r-- 1 taro taro 185 Nov 15 07:45 Cargo.toml
    -rw-r--r-- 1 taro taro 872 Nov 16 08:36 README.md
    -rw-r--r-- 1 taro taro   0 Nov 16 08:36 dog.txt

That's all.
