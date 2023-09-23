#!/bin/bash
set -e

POSTGRES="psql -d ${POSTGRES_DB} --username ${POSTGRES_USER}"

#psql -d ${POSTGRES_DB} --username ${POSTGRES_USER} -f /matrixcare_athena_census.sql

echo "Creating matrixcare_athena_census database steps to be executed!"

$POSTGRES -f /init.sql

