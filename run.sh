#!/bin/bash
xdg-open ./target/android-artifacts/release/apk/hello_maq.apk
read
am start --user 0 -n rust.hello_maq/.MainActivity
