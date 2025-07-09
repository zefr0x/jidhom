_default:
	@just --list

lint-all:
	#!/usr/bin/sh
	for feature in ssr hydrate csr
	do
	cargo clippy --features $feature
	done
	pre-commit run --all-files

todo:
	rg --hidden --glob !.git --glob !{{ file_name(justfile()) }} "( (TODO|FIX|HACK|WARN|PREF): )|(todo!)|(unimplemented!)"

clean:
	cargo clean

generate-db-entities:
	sea-orm-cli generate entity --date-time-crate time --output-dir {{justfile_directory()}}/src/db/src/entities/
