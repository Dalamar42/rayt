RENDER_ARGS := render --width 256 --rays 100  --threads `nproc --all`

.PHONY: help
help:				## Show this help.
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'

.PHONY: regenerate-scenes
regenerate-scenes:		## Renegerate all scene config
	cargo build
	cargo run -- --config config/basic.yaml generate --scene Basic
	cargo run -- --config config/cover.yaml generate --scene Cover
	cargo run -- --config config/cover_with_checker.yaml generate --scene CoverWithChecker
	cargo run -- --config config/cover_with_motion_blur.yaml generate --scene CoverWithMotionBlur
	cargo run -- --config config/next_week_final.yaml generate --scene NextWeekFinal
	cargo run -- --config config/perlin_demo.yaml generate --scene Perlin
	cargo run -- --config config/planets.yaml generate --scene Planets
	cargo run -- --config config/simple_light.yaml generate --scene SimpleLight
	cargo run -- --config config/cornell_box.yaml generate --scene CornellBox
	cargo run -- --config config/cornell_smoke.yaml generate --scene CornellSmoke

.PHONY: render-test
render-test:			## Render all scenes in 'output/test' (low res / low number of rays)
	cargo build --release
	mkdir -p output/test

	target/release/rayt --config config/basic.yaml ${RENDER_ARGS} --output output/test/basic.png
	target/release/rayt --config config/cover.yaml ${RENDER_ARGS} --output output/test/cover.png
	target/release/rayt --config config/cover_with_checker.yaml ${RENDER_ARGS} --output output/test/cover_with_checker.png
	target/release/rayt --config config/cover_with_motion_blur.yaml ${RENDER_ARGS} --output output/test/cover_with_motion_blur.png
	target/release/rayt --config config/next_week_final.yaml ${RENDER_ARGS} --output output/test/next_week_final.png --asset assets/earth.jpg
	target/release/rayt --config config/perlin_demo.yaml ${RENDER_ARGS} --output output/test/perlin_demo.png
	target/release/rayt --config config/planets.yaml ${RENDER_ARGS} --output output/test/planets.png --asset assets/*
	target/release/rayt --config config/simple_light.yaml ${RENDER_ARGS} --output output/test/simple_light.png
	target/release/rayt --config config/cornell_box.yaml ${RENDER_ARGS} --output output/test/cornell_box.png
	target/release/rayt --config config/cornell_smoke.yaml ${RENDER_ARGS} --output output/test/cornell_smoke.png
