# hosts-modifier
> Hosts file manager

# Introduction

Hosts-modifier is a program that aims to help you manage the system hosts file (```/etc/hosts``` on Linux/MacOS and ```C:\Windows\system32\drivers\etc\hosts``` on Windows).
It simplifies the process of adding, removing and listing lines in the file.
(Adding and Removing require administrator or super user privileges in the console or terminal.)

This program is designed for developers to help them manage host files, which they often do. (PHP develpment with XAMPP or any LAMPP stack, Laravel, Blocking sites etc...).

# Installation

1. Download from [Releases page](https://github.com/BrosSquad/hosts/releases/tag/v2.3.0).


2. Building from Source

**Make sure you have golang, make and git installed for your operating system**

```sh
$ git clone https://github.com/BrosSquad/hosts.git hosts && cd hosts
$ git checkout tags/v2.3.0 -b v2.3.0
$ make build VERSION=2.0.3 ENVIRONMENT=production RACE=0
$ make install
```

**Make sure you've added $GOPATH/bin to the $PATH**
That's all.

# Usage

> hosts-modifier is a command line program


#### Get Help

```sh
$ hosts --help
```

#### Add Host

```sh
$ hosts add example.com 127.0.0.1
```

- If you don't pass ip, defaults to "127.0.0.1" (same as example above)

```sh
$ hosts add example.com
```

#### Remove Host

```sh
$ hosts remove example.com
```

#### List Hosts

```sh
$ hosts list
```

## Licence

This program is licensed under the terms of the **GNU GPL v2** only.
