PHONY: db

db:
	pgcli "postgresql://root:alpharius@localhost/trainer_helper"

