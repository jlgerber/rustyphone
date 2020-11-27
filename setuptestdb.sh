#!/usr/bin/bash 
# Run this script to setup the testdb
target/release/teardown-testdb
cd userdb_core && sqlx migrate run && cd ..
target/release/populate-testdb