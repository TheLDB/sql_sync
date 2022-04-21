# sql_sync
By Landon Boles - TheLDB on Github, @lndnNFT on Twitter

## What is sql_sync?
- - - 
*** !!! This was built by a single dev just having fun with Rust, do not trust this for production enviornments. I cannot promise you wont lose data. !!! ***
- -  -

sql_sync is a Rust CLI built using the [clap]() library. 

sql_sync allows you to sync the columns & data of tables between SQL instances. 

Ex. You modified a SQL table on your localhost instance and want to sync it to dev, or even prod servers. 

You run this CLI and choose that, and you can sync the structure & data (if chosen)

How to use?

```bash
cargo install sql_sync

sql_sync --help
```
