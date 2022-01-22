#!/bin/bash
perf stat -d -d -d -r 30 -o gups-512-30 -e `sed -n -e 'H;${x;s/\n/,/g;s/^,//;p;}' perf.aarch64` ./target/release/level-gups
