#!/bin/bash

VERSION_FILE=src/settings.rs
VERSION=$1

if [ -z "$VERSION" ]; then
    echo "ERROR : Bad usage."
    echo -e "\nUsage : $0 VERSION"
    exit 1
fi

sed -i "s/pub const VERSION: &str = \"[^\"]*\";/pub const VERSION: \&str = \"$VERSION\";/" src/settings.rs

exit $?