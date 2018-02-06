#! /bin/sh

cat << EOF > src/schema/db_schema.rs
//! Diesel generated database schema

EOF
diesel print-schema >> src/schema/db_schema.rs