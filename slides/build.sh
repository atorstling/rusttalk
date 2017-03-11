#!/bin/bash

echo -n "Building slides..."
rustdoc slides.md -o . --markdown-playground-url https://play.rust-lang.org/ --html-in-header=inc/header.inc.html --markdown-no-toc
echo " Done."
