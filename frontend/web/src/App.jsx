import React, { useState, useRef, useEffect } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { useDropzone } from 'react-dropzone'
import PixelDemo from './components/PixelDemo'
import './App.css'

// Global variables
let emulator = null
let animationId = null
let canvas = null
let ctx = null
let isInitialized = false

function App() {
  const [status, setStatus] = useState({ message: 'Ready to load ROM', type: 'info' })
  const [isRunning, setIsRunning] = useState(false)
  const [selectedFile, setSelectedFile] = useState(null)
  const canvasRef = useRef(null)
  const fileInputRef = useRef(null)

  // Initialize the application
  useEffect(() => {
    initialize()
  }, [])

  const initialize = async () => {
    try {
      updateStatus('Initializing emulator...', 'loading')
      
      // For now, just show that the frontend is working
      updateStatus('Frontend loaded successfully! WASM integration pending.', 'success')
      
    } catch (error) {
      console.error('Failed to initialize:', error)
      updateStatus('Failed to initialize: ' + error.message, 'error')
    }
  }

  const updateStatus = (message, type = 'info') => {
    setStatus({ message, type })
  }

  // File handling with react-dropzone
  const onDrop = (acceptedFiles) => {
    if (acceptedFiles.length > 0) {
      handleFile(acceptedFiles[0])
    }
  }

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    accept: {
      'application/octet-stream': ['.nes']
    },
    multiple: false
  })

  const handleFile = (file) => {
    setSelectedFile(file)
    updateStatus(`File selected: ${file.name} (WASM integration pending)`, 'info')
  }

  const openFileDialog = () => {
    fileInputRef.current?.click()
  }

  // Emulation control
  const startEmulation = () => {
    setIsRunning(true)
    updateStatus('Start button clicked (WASM integration pending)', 'info')
  }

  const stopEmulation = () => {
    setIsRunning(false)
    updateStatus('Stop button clicked (WASM integration pending)', 'info')
  }

  const resetEmulator = () => {
    updateStatus('Reset button clicked (WASM integration pending)', 'info')
  }

  const stepEmulator = () => {
    updateStatus('Step button clicked (WASM integration pending)', 'info')
  }

  // Keyboard events
  useEffect(() => {
    const handleKeyDown = (event) => {
      const gameKeys = ['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight', 'Enter', 'ShiftRight', 'KeyZ', 'KeyX', 'KeyA', 'KeyS', 'KeyR']
      if (gameKeys.includes(event.code)) {
        event.preventDefault()
      }
    }

    const handleKeyUp = (event) => {
      const gameKeys = ['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight', 'Enter', 'ShiftRight', 'KeyZ', 'KeyX', 'KeyA', 'KeyS', 'KeyR']
      if (gameKeys.includes(event.code)) {
        event.preventDefault()
      }
    }

    const handleWindowBlur = () => {
      if (isRunning) {
        stopEmulation()
      }
    }

    document.addEventListener('keydown', handleKeyDown)
    document.addEventListener('keyup', handleKeyUp)
    window.addEventListener('blur', handleWindowBlur)

    return () => {
      document.removeEventListener('keydown', handleKeyDown)
      document.removeEventListener('keyup', handleKeyUp)
      window.removeEventListener('blur', handleWindowBlur)
    }
  }, [isRunning])

  return (
    <div className="retro-app">
      <div className="retro-container">
        {/* Header */}
        <motion.header 
          className="retro-header"
          initial={{ y: -50, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          transition={{ duration: 0.8, ease: "easeOut" }}
        >
          <h1 className="retro-title">🎮 RNES</h1>
          <p className="retro-subtitle">NES Emulator in Rust + WebAssembly</p>
        </motion.header>
        
        <main className="retro-main">
          {/* Status Panel */}
          <motion.div 
            className="retro-status-panel"
            initial={{ x: -50, opacity: 0 }}
            animate={{ x: 0, opacity: 1 }}
            transition={{ duration: 0.6, delay: 0.2 }}
          >
            <div className={`retro-status-card status-${status.type}`}>
              <h3>Status</h3>
              <p>{status.message}</p>
            </div>
            
            <div 
              {...getRootProps()}
              className={`retro-file-drop-zone ${isDragActive ? 'drag-over' : ''}`}
            >
              <input {...getInputProps()} />
              <div className="drop-zone-content">
                <div className="drop-zone-icon">📁</div>
                <p>
                  {isDragActive 
                    ? 'Drop your .nes file here!' 
                    : 'Drop your .nes file here or click to browse'
                  }
                </p>
                <input 
                  ref={fileInputRef}
                  type="file" 
                  accept=".nes" 
                  onChange={(e) => e.target.files[0] && handleFile(e.target.files[0])}
                  style={{ display: 'none' }}
                />
              </div>
            </div>
          </motion.div>
          
          {/* Emulator Panel */}
          <motion.div 
            className="retro-emulator-panel"
            initial={{ x: 50, opacity: 0 }}
            animate={{ x: 0, opacity: 1 }}
            transition={{ duration: 0.6, delay: 0.4 }}
          >
            <div className="retro-canvas-container">
              <canvas 
                ref={canvasRef}
                id="nes-canvas" 
                width="256" 
                height="240"
                className="retro-canvas"
              />
              <AnimatePresence>
                {!selectedFile && (
                  <motion.div 
                    className="retro-canvas-overlay"
                    initial={{ opacity: 1 }}
                    exit={{ opacity: 0 }}
                    transition={{ duration: 0.3 }}
                  >
                    <div className="overlay-content">
                      <p>Load a ROM to start playing</p>
                    </div>
                  </motion.div>
                )}
              </AnimatePresence>
            </div>
            
            <div className="retro-controls-panel">
              <div className="retro-control-buttons">
                <motion.button 
                  className={`retro-btn retro-btn-success ${!selectedFile ? 'disabled' : ''}`}
                  onClick={startEmulation}
                  disabled={!selectedFile}
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                >
                  <span className="btn-icon">▶️</span>
                  <span className="btn-text">Start</span>
                </motion.button>
                
                <motion.button 
                  className={`retro-btn retro-btn-danger ${!isRunning ? 'disabled' : ''}`}
                  onClick={stopEmulation}
                  disabled={!isRunning}
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                >
                  <span className="btn-icon">⏹️</span>
                  <span className="btn-text">Stop</span>
                </motion.button>
                
                <motion.button 
                  className={`retro-btn retro-btn-secondary ${!selectedFile ? 'disabled' : ''}`}
                  onClick={resetEmulator}
                  disabled={!selectedFile}
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                >
                  <span className="btn-icon">🔄</span>
                  <span className="btn-text">Reset</span>
                </motion.button>
                
                <motion.button 
                  className={`retro-btn retro-btn-secondary ${!selectedFile ? 'disabled' : ''}`}
                  onClick={stepEmulator}
                  disabled={!selectedFile}
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                >
                  <span className="btn-icon">⏭️</span>
                  <span className="btn-text">Step</span>
                </motion.button>
              </div>
              
              <div className="retro-emulator-info">
                <div className="retro-info-card">
                  <h3>Controls</h3>
                  <div className="retro-controls-grid">
                    <div className="retro-control-item">
                      <span className="retro-control-key">↑↓←→</span>
                      <span className="retro-control-desc">D-pad</span>
                    </div>
                    <div className="retro-control-item">
                      <span className="retro-control-key">Enter</span>
                      <span className="retro-control-desc">Start</span>
                    </div>
                    <div className="retro-control-item">
                      <span className="retro-control-key">R.Shift</span>
                      <span className="retro-control-desc">Select</span>
                    </div>
                    <div className="retro-control-item">
                      <span className="retro-control-key">Z</span>
                      <span className="retro-control-desc">A Button</span>
                    </div>
                    <div className="retro-control-item">
                      <span className="retro-control-key">X</span>
                      <span className="retro-control-desc">B Button</span>
                    </div>
                    <div className="retro-control-item">
                      <span className="retro-control-key">A/S</span>
                      <span className="retro-control-desc">Turbo A/B</span>
                    </div>
                    <div className="retro-control-item">
                      <span className="retro-control-key">R</span>
                      <span className="retro-control-desc">Reset</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </motion.div>
          
          {/* Pixel Demo Component */}
          <PixelDemo />
        </main>
      </div>
    </div>
  )
}

export default App
