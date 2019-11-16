# rust-raytracer
Ray tracer in Rust, adapted from [an old C++ project](https://github.com/hippopotamus-prime/raytracer)

## Planned Features
* [ ] Various geometry primitives
  * [ ] Spheres
  * [ ] Cones
  * [ ] Cylinders
  * [x] 2D Polygons
* [ ] Various shading models
  * [ ] Phong
  * [ ] Blinn-Phong
* [ ] Reflection & refraction
* [ ] Point light sources with shadowing
* [ ] Accelerated rendering with K-D trees
* [x] Input in [NFF format](http://paulbourke.net/dataformats/nff/nff1.html)
* [x] Output in [PPM format](http://netpbm.sourceforge.net/doc/ppm.html)

## Usage
Input is from stdin. Output is to a file named *trace.ppm* in the current directory.
```
cargo run < nff/teapot.nff
```
