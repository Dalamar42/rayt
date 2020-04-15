# Rayt

<p float="left">
  <img src="/samples/cornell_box.png" width="450" />
  <img src="/samples/next_week_final.png" width="450" />
</p>

This is a Rust implementation of the ray-tracer from Peter Shirley's excellent [books][books],
`Ray Tracing in One Weekend`, `Ray Tracing: The Next Week`, and `Ray Tracing: The Rest of Your Life`.

This is not by any means a fully featured ray-tracer. Rather, this is something I built in order to learn Rust
and ray-tracing. I am sharing it in case it is useful to others trying to do the same.

## Functionality
Rayt stores and reads scene data in its own format using yaml files. Those are nothing more than a yaml
serialisation of the corresponding Rust objects.

There are two modes of operation described in more detail in the next section.
- `generate` which will generate the yaml file for a scene based on presets in the code
- `render` which will render the scene in the given yaml

Various examples from the books are provided in the form of scenes. The `generate` mode can be used to
quickly regenerate the config files after making changes to them in the code, but for small scenes the files 
can be easily directly edited as well.

## How to run
This code was developed and tested using Rust `1.42`, however it is not using any bleeding edge features of
the language so any reasonably recent stable version should be enough to compile it.

To install the binary in your local environment:
```bash
cargo install --path ${PATH_TO_THIS_REPO} 
```

Help:
```bash
rayt --help
rayt generate --help
rayt render --help
```

Generate the Cornell box scene yaml file:
```bash
rayt --config config/cornell_box.yaml generate --scene CornellBox
```

To render the Cornell box:
```bash
rayt --config config/cornell_box.yaml \
    render --width 512 --rays 1000 --threads 8 --output output/cornell_box.png
```

Some scenes use assets as textures, for example the final scene from the `Ray Tracing: The Next Week` book.
These assets can be passed in using `--asset`:
```bash
rayt --config config/next_week_final.yaml \
    render --width 512 --rays 1000 --threads 8 --asset assets/earth.jpg --output output/next_week_final.png
```

The option `--threads` can be used to control how many threads the renderer should use and the option `--rays`
will determine how many rays (samples) will be taken for each pixel. Approximately 1000 samples should be
enough to produce a decent image with some noise from the provided scenes, but more are needed for a clear
image. The rendering times will increase quite significantly with the number of samples selected and the size
of the image. On my machine rendering `next_week_final.yaml` with 5000 samples, and a width of 1024 pixels
took around 9 hours. Conversely, the `cornell_box.yaml` with a 1000 samples, and a width of 512 finishes in a
few minutes.

A `Makefile` is included with some convenience targets:
- `make regenerate-scenes` will create all the scene config yaml files
- `make render-test` will render all scenes using a moderate resolution and number of rays and put the
  generated images in `output/test`
- `make cornell-test` is the same as `make render-test`, but only for the Cornell box
- `make regenerate-samples` will create high-resolution / high-ray-count versions of the Cornell box and the
  final image from book 2 and put them in `output/samples`

## Samples
The directory `samples` contains images generated with high number of rays, specifically 5000 rays per pixel,
to showcase what kinds of images `rayt` can render.

## Assets

The included assets were taken from the following sources:

- _blue_marble.jpg_ - [NASA Visible Earth][nasa-visible-earth]
- _earth.jpg_ - [NASA Visible Earth][nasa-visible-earth]
- _earth_night.jpg_ - [NASA Visible Earth][nasa-visible-earth]
- _jupiter.jpg_ - [NASA Cassini][nasa-cassini]
- _mars.jpg_ - [USGS][usgs-mars]
- _moon.jpg_ - [USGS][usgs-moon]

## License

See the LICENSE file that is included with this repository.

[books]: https://github.com/RayTracing/raytracing.github.io
[nasa-visible-earth]: https://visibleearth.nasa.gov/
[nasa-cassini]: https://solarsystem.nasa.gov/missions/cassini/galleries/
[usgs-mars]: https://astrogeology.usgs.gov/search/details/Mars/Viking/MDIM21/Mars_Viking_MDIM21_ClrMosaic_global_232m/cub
[usgs-moon]: https://astrogeology.usgs.gov/search/map/Moon/Clementine/UVVIS/Lunar_Clementine_UVVIS_750nm_Global_Mosaic_118m_v2