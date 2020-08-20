# hosts-modifier
> Hosts file manager

# Introduction

Hosts-modifier is program that aims to help you manager system hosts file (/etc/hosts on linux and C:\Windows\system32\drivers\etc\hosts on Windows).
It simplifies process of adding, removing and listing lines in the file. 
(Adding and Removing required administrator privileges to console)

This program is designed for developers to help them manage host files, which they often do. (Php develpment with XAMPP or any LAMPP stack, Laravel, Blocking sites etc...).


# Installing

Currently only supported method of installing is building it from source. (Don't worry, go building is simple)
**Make sure you have golang installed for your operating system**

```sh
go get github.com/BrosSquad/hosts-modifier
```
**Make sure you've added $GOPATH/bin to the $PATH**
That's all.

# Usage

> hosts-modifier is command line program


#### Get Help

```sh
$ hosts --help
```

#### Add Host

```sh
$ hosts add --host example.com --ip 127.0.0.1
```


#### Remove Host

```sh
$ hosts remove --host example.com --ip 127.0.0.1
```

#### List Hosts

```sh
$ hosts list
```

## Disclaimer

This program works only on Linux and on Windows, MacOS is not supported and it will never be, even though code for linux will (maybe) work on MacOS.
Sorry apple duds, install linux its better. We wont accept any pull requests related to MacOS and all Issues related with MacOS will be closed. 
Other contributions like list filtering and other ideas are all welcome and will most likely be accepted and merged into main branch.

## Licence

only **GNU GPL v2**

