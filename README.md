# noventis-api
School project

## Prerequires

Install [Rust](https://www.rust-lang.org/)

Setup [postgresql](https://wixelhq.com/blog/how-to-install-postgresql-on-ubuntu-remote-access)

```
User : dbuser
Password: root
host: localhost
port :5432
```

Install diesel-cli

## Running the app

### First run:
 ``` rustup default nightly ```

### Running the app
``` cargo run ```

Apply migrations :

``` diesel migration run ```

Undo migrations : 

``` diesel migration redo ```