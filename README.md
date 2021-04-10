# rust-raytracer
Port of my old [old C++ ray tracer](https://github.com/hippopotamus-prime/raytracer)
to Rust, done mostly as an exercise to learn the language.

## Features
* Various geometry primitives
  * Spheres
  * Cones
  * Cylinders
  * 2D Polygons
* Various shading models
  * Phong
  * Blinn-Phong
* Reflection & refraction
* Point light sources with shadowing
* Accelerated rendering with bounding volume hierarchies (K-D trees)
* Input in [NFF format](https://github.com/erich666/StandardProceduralDatabases/blob/master/NFF.TXT)
* Output in [PPM format](http://netpbm.sourceforge.net/doc/ppm.html)

## Usage
Input is from stdin. Output is to a file named *trace.ppm* in the current directory.
```
cargo run < nff/teapot.nff
```

The ray tracer should be compatible with all NFF files in the [Standard Procedural Databases](https://github.com/erich666/StandardProceduralDatabases).

## Example Output
![Obligatory Utah Teapot](https://i.imgur.com/8JQMPjq.png)
