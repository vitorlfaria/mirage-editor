# MIRAGE EDITOR

Command Line Image Editor

## 🚀 Start

These instructions will allow you to get a working copy of the project on your local machine.


### 📋 Prerequisites

You need Rust language and Cargo installed

### 🔧 Usage

There is two ways to use the application.

One command:
```
cargo run [subcommand] [amount (for commands that need)] [infile] [outfile]
```
Example:
```
cargo run blur 2.5 image.png blurred.png
```
---

Multiple commands:
```
cargo run [infile] [outfile] [commands]
```
Example:
```
cargo run image.png blurred.png blur 2.5 invert rotate 90 grayscale
```

## ✨ Commands
Obs: All the commands that needs amount, the amount has to be right after the command to work.

---
### blur
It blurs the image and needs a float number as amount.

---
### brighten
It lightens and darkens the image and needs a float number as amount, positive values will lighten the image and negative numbers will darken the image.

---
### invert
Inverts the colors of the image.

---
### grayscale
Turns the image into grayscale.

---
### crop
Crops the image and needs four unsigned integer arguments: x, y, width and height.


## 🛠️ Built with
* [Rust](https://www.rust-lang.org)

## ✒️ Authors
* **Vitor Lacerda** - *Dev* - [vitorlfaria](https://github.com/vitorlfaria)

## 📄 License
* MIT

