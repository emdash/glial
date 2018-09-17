# An exploration of OpenGL and Rust

This repo is something of a sandbox for exploring GPU programming. I
chose Rust and Glium precisely because they offered the perfect
sandbox for this kind of project.

## How to use this repo

Treat this repo like a tutorial. Each commit is a small incremental
step, and represents some lesson learned. Each diff should have some
meaning. I started with a simple glium tutorial that that draws a 2D
triangle. I followed it more-or-less faithfully for a few chapters, before 
striking off in my own direction.

## Why Glium?

Glium is a safe API that abstracts over multiple OpenGL versions,
while at the same preserving the "feel" of raw OpenGL. I don't want a
dumbed down API. I just don't want to waste time trouble-shooting dumb
mistakes with the OpenGl API.

I don't want to muck around with the complexities of using OpenGL
directly. There's a buch of nonsense around "loading" and whatnot that
is needless distraction from the core principles. Vulkan might be
better, but I don't have hardware that can run Vulkan, and I don't
expect to need much beyond GLESv2 in my professional work any time
soon. But GLESv2 would be a huge help right now.

## Goals

- Do something non-trivial in Rust.
- Learn "modern-ish" GL programming without getting too bogged down in
  the pitfalls of the C API.
- Learn how game engines and other abstractions over the GL API
  actually work under the hood.
  - I have looked inside the Kivy and ThreeJS and found it somewhat inscrutable.
  - As my own code has evolved, it has become similarly inscrutable. Now I hunderstand why.
- Get a "feel" for what an idiomatic Rust library is like to use.
