# RNES Web Frontend

A modern web frontend for the RNES NES emulator built with Vite and WebAssembly.

## Features

- 🎮 Modern, responsive UI design
- 📁 Drag & drop ROM loading
- ⚡ Fast development with Vite
- 🎨 Beautiful animations and transitions
- 📱 Mobile-friendly interface
- 🔧 Hot module replacement for development

## Prerequisites

- Node.js 18+ 
- Rust 1.89+
- wasm-pack

## Development

1. **Install dependencies:**
   ```bash
   npm install
   ```

2. **Build WASM module:**
   ```bash
   npm run build-wasm
   ```

3. **Start development server:**
   ```bash
   npm run dev
   ```

4. **Or build WASM and start dev server in one command:**
   ```bash
   npm run dev-full
   ```

The development server will be available at `http://localhost:3000`.

## Building for Production

1. **Build WASM module:**
   ```bash
   npm run build-wasm
   ```

2. **Build the web application:**
   ```bash
   npm run build
   ```

3. **Preview the production build:**
   ```bash
   npm run preview
   ```

## Project Structure

```
frontend/web/
├── src/
│   ├── main.js          # Main application logic
│   └── style.css        # Styles and animations
├── pkg/                 # WASM build output (generated)
├── public/              # Static assets
├── index.html           # Main HTML file
├── vite.config.js       # Vite configuration
└── package.json         # Dependencies and scripts
```

## Controls

- **Arrow Keys:** D-pad
- **Enter:** Start
- **Right Shift:** Select
- **Z:** A Button
- **X:** B Button
- **A/S:** Turbo A/B
- **R:** Reset

## Browser Support

This application requires a modern browser with WebAssembly support:
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Troubleshooting

### WASM Loading Issues
If you encounter WASM loading issues, make sure:
1. The WASM module is built (`npm run build-wasm`)
2. You're using a supported browser
3. The development server is running with proper CORS headers

### Performance Issues
- Use the latest version of your browser
- Ensure hardware acceleration is enabled
- Close other resource-intensive applications

## Contributing

1. Make your changes in the `src/` directory
2. Test with `npm run dev`
3. Build for production with `npm run build`
4. Submit a pull request

## License

This project is licensed under the MIT License.
