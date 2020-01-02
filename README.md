# rust-raytracer
Ray tracer in Rust, adapted from [an old C++ project](https://github.com/hippopotamus-prime/raytracer)

## Planned Features
* [x] Various geometry primitives
  * [x] Spheres
  * [x] Cones
  * [x] Cylinders
  * [x] 2D Polygons
* [x] Various shading models
  * [x] Phong
  * [x] Blinn-Phong
* [x] Reflection & refraction
* [x] Point light sources with shadowing
* [ ] Accelerated rendering with K-D trees
* [x] Input in [NFF format](https://github.com/erich666/StandardProceduralDatabases/blob/master/NFF.TXT)
* [x] Output in [PPM format](http://netpbm.sourceforge.net/doc/ppm.html)

## Usage
Input is from stdin. Output is to a file named *trace.ppm* in the current directory.
```
cargo run < nff/teapot.nff
```

The ray tracer should be compatible with all NFF files in the [Standard Procedural Databases](https://github.com/erich666/StandardProceduralDatabases).
