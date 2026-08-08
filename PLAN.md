## PLAN

### Short-term goals
- Define `src/types.rs` for any special types used in the project.
  - RenderContext
  - ColorIntensity, Colors 
- Define `src/traits.rs` to define the interfaces.
  - Effect
  - GpuEffect
  - ControllParameters
- Consider CPU-and GPU-rendering in parallel from the start.
- Implement a MVP that opens a window that displays a simple oscilloscope view of the audio input rendered on the CPU.

### Long term goals (unfinished)
- Render all AVS effects on CPU _and_ on GPU
- Message-passing communication to control RVS locally or over network \
  -> take advantage of Rust's algebraic type types. 
- Render them as close to the original AVS implementation as possible

### Small-ish set of dependencies
- Winit - open a window that display the framebuffer
- cpal - audio capture 
- pixels/glow - rendering (do we need this?)
- argh - cli-parsing
- ... maybe not tokio - async for message passing comes later
