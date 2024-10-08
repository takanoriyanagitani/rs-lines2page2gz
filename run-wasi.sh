#!/bin/sh

input="${ENV_INPUT:-/dev/stdin}"
output="${ENV_OUTPUT:-./out.gz}"
offset="${ENV_OFFSET:-./offset.txt}"

bytecode=./rs-lines2page2gz.wasm
nativebin=./rs-lines2page2gz

rtime="${ENV_WASI_RUNTIME:-wazero}"

case "${rtime}" in
	iwasm)
		cat "${input}" |
			iwasm \
				--env=ENV_CHUNK_SIZE="${ENV_CHUNK_SIZE:-10}" \
				"${bytecode}" 2>"${offset}" |
			dd \
				if=/dev/stdin \
				of="${output}" \
				conv=fsync \
				status=progress
		;;

	native)
		cat "${input}" |
			"${nativebin}" 2>"${offset}" |
			dd \
				if=/dev/stdin \
				of="${output}" \
				conv=fsync \
				status=progress
		;;

	*)
		cat "${input}" |
			"${rtime}" \
				run \
				--env ENV_CHUNK_SIZE=${ENV_CHUNK_SIZE:-10} \
				"${bytecode}" \
				2>"${offset}" |
			dd \
				if=/dev/stdin \
				of="${output}" \
				conv=fsync \
				status=progress
		;;
esac
