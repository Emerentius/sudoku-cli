#!/usr/bin/env bash

if [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
	mv target/release/rudoku rudoku.exe
	7z a "rudoku-$TRAVIS_TAG-$TRAVIS_OS_NAME-x86_64.zip" rudoku.exe
else
	mv target/release/rudoku rudoku
	tar -czf "rudoku-$TRAVIS_TAG-$TRAVIS_OS_NAME-x86_64.tar.gz" rudoku
fi
