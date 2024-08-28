
# s3li

Terminal user interface for s3, heavily inspired by lazygit, focused on making interacting with s3 (and aws) easier

# Features currently implemented

* manage aws accounts stored within the credentials file under ~/.aws/credentials
  * switch between accounts
  * change region
  * add/edit property of an account (need to implement reading from clipboard to ease the editing)
  * refresh credentials manually
* manage s3 buckets
  * choose which bucket to explore
  * navigate files and folders of the selected bucket
  * download files to desired location (defaults to current working directory)
  * select multiple files
  * download multiple files
  * download multiple folders (and all the files within)
* global
  * add area to display keybinds of currently selected section

# Coming features

## Accounts

* add accounts directly from the interface
* ease the editing or creation of an account and its properties
* read credentials file in a path that is different from the default one
* find out how sso works and if it can be integrated

## Preferences

* add a way to store preferences (config file might be enough, should not store much anyway)
* add preferences section to edit application wide properties (credentials file path, default account, ecc.)

## Explorer

* view files and folder properties/permissions
* copy selected files/folders to another bucket
* preview of simple files, txt, json, csv ecc. (might even add parquet files, since it's what I am currently working with)

## S3

* copy entire bucket contents to another
* allow for cross account bucket operation (I'm gonna be honest, I might never add a delete option for buckets)

# Installation

Still WIP, for now you can run the project with `cargo run` or build and run the executable.

I did not test this with PowerShell, should work with wsl

# Motivation

I am annoyed by the aws console, I'd rather do the same from the terminal. Also I am learning Rust this way.
