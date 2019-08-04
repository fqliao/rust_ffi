#!/usr/bin/env bash

LANG=zh_CN.UTF-8
##############################################################################
##
##  start up script for UN*X
##
##############################################################################

# set shell path
dirpath="$(cd "$(dirname ${0})" && pwd)"
cd ${dirpath}

# @function: output information log
# @param: content: information message
function LOG_INFO_ORANGE()
{
    local content=${1}
    echo -e "\033[33m"${content}"\033[0m"
    echo -e
}
function LOG_INFO_BLUE()
{
    local content=${1}
    echo -e "\033[34m"${content}"\033[0m"
}

# define output dir variable
rust_outdir="target/debug/"
rust_cdy="rust"
other_outdir="other_language/"

# build and test rust
cargo build
cargo test

# C++: compile and execute
LOG_INFO_BLUE "C++:"
g++ -c --std=c++11 ${other_outdir}/c_demo.cpp -o ${other_outdir}/c_demo.o
g++ --std=c++11 ${other_outdir}/c_demo.o -L${rust_outdir} -l${rust_cdy} -o ${other_outdir}/c_demo
LOG_INFO_ORANGE "$(LD_LIBRARY_PATH=${rust_outdir} ./${other_outdir}c_demo)"

# Java
LOG_INFO_BLUE "Java:"
javac -d ${other_outdir} ${other_outdir}*.java
LOG_INFO_ORANGE "$(LD_LIBRARY_PATH=${rust_outdir} java -cp ${other_outdir} JavaDemo)"

# Python
LOG_INFO_BLUE "Python:"
LOG_INFO_ORANGE "$(LD_LIBRARY_PATH=${rust_outdir} python ${other_outdir}python_demo.py)"

# Nodejs
LOG_INFO_BLUE "Nodejs:"
LOG_INFO_ORANGE "$(LD_LIBRARY_PATH=${rust_outdir} node ${other_outdir}nodejs_demo.js)"

rm -rf ${rust_outdir}rust-*