#!/usr/bin/bash 
# Run this script to setup the testdb
target/release/teardown-testdb
sqlx migrate run
target/release/populate-testdb