#!/bin/sh

input=~/Downloads/PNG_transparency_demonstration_1.png

export ENV_I_PNG_FILENAME="${input}"
export ENV_O_PPM_FILENAME=./out.ppm

run_native(){
	./rs-png2pnm
}

run_wazero(){
	wazero \
		run \
		-env ENV_I_PNG_FILENAME=/iguest.d/PNG_transparency_demonstration_1.png \
		-env ENV_O_PPM_FILENAME=/guest.d/out.ppm \
		-mount "${PWD}:/guest.d" \
		-mount "$HOME/Downloads:/iguest.d" \
		./rs-png2pnm.wasm
}

which wazero | fgrep -q wazero || run_native
which wazero | fgrep -q wazero && run_wazero

file "${ENV_I_PNG_FILENAME}"
file "${ENV_O_PPM_FILENAME}"

xxd "${ENV_O_PPM_FILENAME}" | head -16384 | tail
