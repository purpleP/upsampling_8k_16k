kernel:
	python3 kernel_remez.py

test:
	cargo run -- ~/Downloads/2000kHz_sin_16bit_8kHz.wav
	ffmpeg -i out.wav -lavfi showspectrumpic out.png
