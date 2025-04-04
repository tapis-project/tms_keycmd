#!/bin/sh
# Build and package the TMS KeyCmd utility
# This script builds a release version and packages up all files
#   into a compressed tar file. The generated tar file is located
#   at the top level of the repo.

PrgName=$(basename "$0")

USAGE="Usage: $PrgName"

# Check number of arguments
if [ $# -ne 0 ]; then
  echo "$USAGE"
  exit 1
fi

# Determine absolute path to location from which we are running
#  and change to that directory.
export RUN_DIR=$(pwd)
export PRG_RELPATH=$(dirname "$0")
cd "$PRG_RELPATH"/. || exit
export PRG_PATH=$(pwd)

# Determine the app version
VERS=$(cargo pkgid | cut -d "#" -f2)

# Cleanup and define temp staging area
rm -fr /tmp/tms_keycmd_staging
STG_DIR=/tmp/tms_keycmd_staging/tms-keycmd-${VERS}

# Path in current dir to final tar archive to be created
TGZ_FILE="tms-keycmd-${VERS}.tgz"
TGZ_PATH="$RUN_DIR/$TGZ_FILE"

# Path to tgz file in rpm source directory
RPM_TGZ_PATH="${PRG_PATH}/../packaging/rpmbuild/SOURCES/${TGZ_FILE}"

# List of files from the repo top level that are to be included
TOP_FILES="log4rs.yml tms_keycmd.toml tms_keycmd.sh README.md LICENSE"

# Build the executable from the top level of the repo
cd ..
echo "Building executable from directory: $(pwd)"
cargo build --release

# Create a staging directory and empty log file
mkdir -p $STG_DIR/logs
touch $STG_DIR/logs/tms_keycmd.log
# Copy files to staging dir
cp $TOP_FILES target/release/tms_keycmd $STG_DIR

# Change to staging dir and set permissions
cd "$STG_DIR"/. || exit
chmod 600 $TOP_FILES
chmod 700 tms_keycmd tms_keycmd.sh

# Create the tar file in the current working directory of invoking user
echo "Creating compressed tar archive at path: $TGZ_PATH"
if [ -f "$TGZ_PATH" ]; then
  rm "$TGZ_PATH"
fi
# Move up one level so our final tar file is all under one dir when unpacked.
cd "$STG_DIR"/.. || exit
tar -czf "$TGZ_PATH" tms-keycmd-${VERS}

# Copy the tgz file to the rpmbuild source directory
if [ -f "$RPM_TGZ_PATH" ]; then
  rm "${RPM_TGZ_PATH}"
fi
cp "$TGZ_PATH" "${RPM_TGZ_PATH}"

# Switch back to current working directory of invoking user
cd "$RUN_DIR"
