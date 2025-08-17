# RNES Web Frontend

## Overview

The RNES Web Frontend is a browser-based NES emulator built with Rust and WebAssembly. It provides a simple, modern interface for running NES games directly in the browser.

## Features

- 🎮 Full NES emulation in the browser
- 🖥️ Canvas 2D rendering
- ⌨️ Keyboard input support
- 📁 File upload for ROM loading
- 🔧 Step-by-step execution
- 📱 Responsive design

## Architecture

### Core Components

- **WebNesEmulator**: Main WASM interface to the Rust emulator
- **Canvas Rendering**: Direct frame buffer to canvas rendering
- **File Handling**: ROM loading via file input
- **Input Management**: Keyboard event handling

### Technology Stack

- **Rust**: Core emulator logic
- **WebAssembly**: Compiled Rust code for browser execution
- **HTML5 Canvas**: Graphics rendering
- **JavaScript**: UI and browser integration
- **wasm-pack**: Build tool for Rust to WASM compilation

## Building

### Prerequisites

- Rust 1.89+
- wasm-pack (installed automatically by build script)
- Python 3 (for local development server)

### Build Process

1. Navigate to the web frontend directory:
   ```bash
   cd frontend/web
   ```

2. Run the build script:
   ```bash
   ./build.sh
   ```

The build script will:
- Install wasm-pack if needed
- Compile Rust code to WebAssembly
- Generate JavaScript bindings
- Copy HTML and assets
- Create a local development server

### Running

After building, start the local server:

```bash
cd pkg
python3 server.py
```

Then open your browser and navigate to `http://localhost:8080`.

## Usage

### Loading ROMs

1. Click the "Load ROM" button
2. Select a .nes file from your computer
3. The ROM will be loaded into the emulator

### Controls

| NES Button | Keyboard |
|------------|----------|
| D-pad | Arrow Keys |
| Start | Enter |
| Select | Right Shift |
| A | Z |
| B | X |
| A (Turbo) | A |
| B (Turbo) | S |
| Reset | R |

### Emulator Controls

- **Start**: Begin emulation
- **Stop**: Pause emulation
- **Reset**: Reset the emulator
- **Step**: Execute one CPU cycle

## Implementation Details

### WASM Interface

The main interface is exposed through the `WebNesEmulator` struct:

```rust
#[wasm_bindgen]
pub struct WebNesEmulator {
    emulator: Arc<Mutex<Emulator>>,
    canvas: HtmlCanvasElement,
    running: bool,
}
```

### Frame Rendering

Frames are rendered by:
1. Getting the frame buffer from the PPU
2. Converting pixels to RGBA bytes
3. Creating an ImageData object
4. Drawing to the canvas

```javascript
function renderFrame() {
    const frameBuffer = emulator.get_frame_buffer();
    const imageData = ctx.createImageData(256, 240);
    const data = imageData.data;
    
    for (let i = 0; i < frameBuffer.length; i++) {
        data[i] = frameBuffer[i];
    }
    
    ctx.putImageData(imageData, 0, 0);
}
```

### Animation Loop

The emulator runs in a requestAnimationFrame loop:

```javascript
function animationLoop() {
    if (!emulator || !emulator.is_running()) return;
    
    emulator.step();
    renderFrame();
    
    animationId = requestAnimationFrame(animationLoop);
}
```

## Performance Considerations

### Optimization

- **WASM Compilation**: Uses `opt-level = "s"` for smaller binary size
- **LTO**: Link-time optimization enabled
- **Single Codegen Unit**: Reduces binary size
- **Panic Abort**: Smaller panic handling

### Memory Management

- Frame buffers are copied to avoid lifetime issues
- Proper mutex usage for thread safety
- Efficient pixel format conversion

## Browser Compatibility

- **Chrome/Edge**: Full support
- **Firefox**: Full support
- **Safari**: Full support
- **Mobile Browsers**: Basic support (no touch input yet)

## Future Enhancements

### Planned Features

- [ ] WebGL rendering for better performance
- [ ] Audio output via Web Audio API
- [ ] Touch input support for mobile
- [ ] Save/load states
- [ ] Debug tools and memory viewer
- [ ] Drag & drop ROM loading
- [ ] Gamepad support

### Technical Improvements

- [ ] Audio synchronization
- [ ] Frame rate limiting
- [ ] Input lag reduction
- [ ] Memory usage optimization
- [ ] Better error handling

## Development

### Project Structure

```
frontend/web/
├── src/
│   └── lib.rs              # Main WASM interface
├── index.html              # Web interface
├── build.sh               # Build script
├── Cargo.toml             # Rust dependencies
└── README.md              # Documentation
```

### Adding Features

1. **New Emulator Features**: Add to `lib.rs` and expose via WASM bindings
2. **UI Changes**: Modify `index.html` and associated JavaScript
3. **Rendering**: Update frame rendering logic in JavaScript
4. **Input**: Add new input methods in JavaScript

### Debugging

- Use browser developer tools for JavaScript debugging
- Enable Rust logging with `set_log_level('debug')`
- Check WebAssembly console output for Rust errors
- Monitor canvas performance in browser dev tools

## Troubleshooting

### Common Issues

1. **WASM Loading Fails**: Check browser console for errors
2. **ROM Loading Fails**: Ensure file is a valid .nes ROM
3. **Performance Issues**: Check browser performance tools
4. **Input Not Working**: Verify keyboard event handling

### Debug Mode

Enable debug logging:
```javascript
set_log_level('debug');
```

## Contributing

1. Follow Rust coding standards
2. Test on multiple browsers
3. Ensure mobile compatibility
4. Update documentation
5. Add tests for new features

## License

MIT License - see main project license.
