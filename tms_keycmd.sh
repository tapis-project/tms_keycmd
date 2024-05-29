#!/bin/bash
#
# Basic wrapper script for running TMS KeyCmd program from
#   an arbitrary location.
#
# Determine current directory
RUN_DIR="$(pwd -P)"

# Determine absolute path to location from which we are running.
# Note that output is sent to /dev/null in the unlikely case that the cd command
#   generates extraneous output that would interfere with the surrounding $()
# The -P option on pwd is for better handling of symlinks.
# The "--"  on cd is in case the directory begins with "-". The "--"
#   tells the shell that there are no more options.
PRG_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

# Change to the directory where script is installed.
cd $PRG_PATH

# Run the TMS KeyCmd program with all arguments passed on the command line.
# Note that "$@" is used so that original order and grouping is perserved.
#   For example, input arguments might look like this: "a b 'c d'"
./tms_keycmd "$@"

# Return to original directory
cd $RUN_DIR
