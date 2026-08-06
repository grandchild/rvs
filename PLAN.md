### 1. Functional Minimal Prototype
- Window – Create a window and run a render loop that draws something on each frame.
- Audio‑In – Capture live audio (microphone, line‑in) using cpal. No audio output.
- CPU rendering initially, swappable GPU support later considered from the start.

### 2. Core Architecture
- src/types.rs
  - RenderContext – context passed to each effect's render call, containing all information & objects needed for a render pass.
  - Effect – trait with fn render(&self, ctx: &mut RenderContext);.
  - GpuRender – placeholder trait for future GPU render implementations, effects may impl or not.
- basic visualiser (waveform) that consumes the audio buffer and draws to the window.

### 3. Lean Dependency Set
│ Concern         │ Crate                                  │
├─────────────────┼────────────────────────────────────────┤
│ Window & events │ winit                                  │
│ Audio capture   │ cpal (cross‑platform, no async needed) │
│ Rendering (CPU) │ pixels (or glow – simple 2‑D drawing)  │
│ CLI parsing     │ argh                                   │
│ Async runtime   │ none (no Tokio)                        │

### 4. Simple Message‑Passing API
Expose tiny public API that receives control messages (e.g. Start, Stop, SetParam { name, value }).

### 6. Prioritise Iteration Speed Over Perfection
- Follow “Don’t let perfect be the enemy of good.” Deliver a working visualiser that shows live audio in a window.
- De‑prioritise CI, exhaustive testing, pixel‑perfect shader optimisation or now.
