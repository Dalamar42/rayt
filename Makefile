RAYT := target/release/rayt
NPROCS = $(shell nproc)
TEST_ARGS = render --width 256 --rays 100 --threads $(NPROCS)

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
	cargo run -- --config config/cornell_metal.yaml generate --scene CornellMetal
	cargo run -- --config config/cornell_sphere.yaml generate --scene CornellSphere

.PHONY: render-test
render-test:			## Render all scenes in 'output/test' (low res / low number of rays)
	cargo build --release
	mkdir -p output/test

	$(RAYT) --config config/basic.yaml $(TEST_ARGS) --output output/test/basic.png
	$(RAYT) --config config/cover.yaml $(TEST_ARGS) --output output/test/cover.png
	$(RAYT) --config config/cover_with_checker.yaml $(TEST_ARGS) --output output/test/cover_with_checker.png
	$(RAYT) --config config/cover_with_motion_blur.yaml $(TEST_ARGS) --output output/test/cover_with_motion_blur.png
	$(RAYT) --config config/next_week_final.yaml $(TEST_ARGS) --output output/test/next_week_final.png --asset assets/earth.jpg
	$(RAYT) --config config/perlin_demo.yaml $(TEST_ARGS) --output output/test/perlin_demo.png
	$(RAYT) --config config/planets.yaml $(TEST_ARGS) --output output/test/planets.png --asset assets/*
	$(RAYT) --config config/simple_light.yaml $(TEST_ARGS) --output output/test/simple_light.png
	$(RAYT) --config config/cornell_box.yaml $(TEST_ARGS) --output output/test/cornell_box.png
	$(RAYT) --config config/cornell_smoke.yaml $(TEST_ARGS) --output output/test/cornell_smoke.png
	$(RAYT) --config config/cornell_metal.yaml $(TEST_ARGS) --output output/test/cornell_metal.png

.PHONY: cornell-test
cornell-test:			## Render cornell box in 'output/test' (low res / low number of rays)
	cargo build --release
	mkdir -p output/test

	$(RAYT) \
		--config config/cornell_box.yaml \
		render \
		--width 512 \
		--rays 1000 \
		--threads $(NPROCS) \
		--output output/test/cornell_box.png
