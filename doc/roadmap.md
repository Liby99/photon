# The Roadmap for Photon

We want to create a tool for 3D Modeling and Rendering, using Electron and Web Frameworks to empower an extensible UI system, and also Rust/C++ native apps to increase the rendering speed.

## Milestones

- [ ] Build Electron & Rust Framework that can load an Electron App with Rust acting as a renderer
- [ ] Ability to Render a Static Scene in Rust using OpenGL in Rust
- [ ] Ability to Control the Camera Position in JavaScript (by passing the information to Rust)
- [ ] Ability to Abstract UI Component (like RenderView) in JavaScript and add other UI Components (Pane based).
- [ ] Implement a Raytracer and be able to switch between GL Renderer & RayTracer
- [ ] Implement gradual RayTracing so that User can see continuous images (Separate threads if possible)
- [ ] Ability to tweak render settings in JavaScript. Implement Tiling for RayTracer to render details. Separate threads if possible
- [ ] Move from Static Scene to Dynamic Scene that can be controlled by Rust Commands. Like Generate a Sphere or move a Cube or something like that.
- [ ] Build link between JavaScript and Rust so that we can pass commands from JavaScript to Rust. Design an API for this. Build Terminal UI in JavaScript.
- [ ] Build JavaScript Playground so that User can Load JS Files or Write JS Files to generate Scenes.
- [ ] Add Timeline support so that users can make animations.
- [ ] Add Plugin System so that Rust-Neon Dylib could be imported from JavaScript. Make Renderer Configurable.

## Thoughts