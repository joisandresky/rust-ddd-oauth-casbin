postgresconn=postgres://postgres:@host.docker.internal:5432/rust-ddd-oauth-casbin

.PHONY: build-local

build-local:
	docker build --build-arg DB_URL=${postgresconn} -t rust-ddd-oauth-casbin .

