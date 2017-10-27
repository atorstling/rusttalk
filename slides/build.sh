#!/bin/bash
url='http://play.rust-lang.org'
#url='http://play.integer32.org'
rustdoc -Zunstable-options slides.md -o . --playground-url ${url} --html-in-header=inc/header.inc.html --markdown-no-toc
