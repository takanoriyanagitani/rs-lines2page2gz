#!/bin/sh

addr=127.0.0.1
port=10089
loc="./target"

miniserve \
	--port ${port} \
	--interfaces "${addr}" \
	"${loc}"
