import './style.css';

console.log('RNES Web Frontend loaded successfully!');

// Global variables
let emulator = null;
let animationId = null;
let canvas = null;
let ctx = null;
let isInitialized = false;

// DOM elements
const elements = {
    statusText: document.getElementById('status-text'),
    fileDropZone: document.getElementById('file-drop-zone'),
    romFile: document.getElementById('rom-file'),
    startBtn: document.getElementById('start-btn'),
    stopBtn: document.getElementById('stop-btn'),
    resetBtn: document.getElementById('reset-btn'),
    stepBtn: document.getElementById('step-btn'),
    canvasOverlay: document.getElementById('canvas-overlay'),
    nesCanvas: document.getElementById('nes-canvas')
};

// Initialize the application
async function initialize() {
    try {
        updateStatus('Initializing emulator...', 'loading');
        
        // For now, just show that the frontend is working
        updateStatus('Frontend loaded successfully! WASM integration pending.', 'success');
        
        // Set up event listeners
        setupEventListeners();
        
    } catch (error) {
        console.error('Failed to initialize:', error);
        updateStatus('Failed to initialize: ' + error.message, 'error');
    }
}

// Set up event listeners
function setupEventListeners() {
    // File drop zone events
    elements.fileDropZone.addEventListener('click', () => elements.romFile.click());
    elements.fileDropZone.addEventListener('dragover', handleDragOver);
    elements.fileDropZone.addEventListener('dragleave', handleDragLeave);
    elements.fileDropZone.addEventListener('drop', handleDrop);
    
    // File input
    elements.romFile.addEventListener('change', handleFileSelect);
    
    // Control buttons
    elements.startBtn.addEventListener('click', startEmulation);
    elements.stopBtn.addEventListener('click', stopEmulation);
    elements.resetBtn.addEventListener('click', resetEmulator);
    elements.stepBtn.addEventListener('click', stepEmulator);
    
    // Keyboard events
    document.addEventListener('keydown', handleKeyDown);
    document.addEventListener('keyup', handleKeyUp);
    
    // Window events
    window.addEventListener('blur', handleWindowBlur);
}

// File handling
function handleDragOver(event) {
    event.preventDefault();
    elements.fileDropZone.classList.add('drag-over');
}

function handleDragLeave(event) {
    event.preventDefault();
    elements.fileDropZone.classList.remove('drag-over');
}

function handleDrop(event) {
    event.preventDefault();
    elements.fileDropZone.classList.remove('drag-over');
    
    const files = event.dataTransfer.files;
    if (files.length > 0) {
        handleFile(files[0]);
    }
}

function handleFileSelect(event) {
    const file = event.target.files[0];
    if (file) {
        handleFile(file);
    }
}

async function handleFile(file) {
    updateStatus(`File selected: ${file.name} (WASM integration pending)`, 'info');
}

// Emulation control
function startEmulation() {
    updateStatus('Start button clicked (WASM integration pending)', 'info');
}

function stopEmulation() {
    updateStatus('Stop button clicked (WASM integration pending)', 'info');
}

function resetEmulator() {
    updateStatus('Reset button clicked (WASM integration pending)', 'info');
}

function stepEmulator() {
    updateStatus('Step button clicked (WASM integration pending)', 'info');
}

// Input handling
function handleKeyDown(event) {
    // Prevent default for game keys
    const gameKeys = ['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight', 'Enter', 'ShiftRight', 'KeyZ', 'KeyX', 'KeyA', 'KeyS', 'KeyR'];
    if (gameKeys.includes(event.code)) {
        event.preventDefault();
    }
}

function handleKeyUp(event) {
    // Prevent default for game keys
    const gameKeys = ['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight', 'Enter', 'ShiftRight', 'KeyZ', 'KeyX', 'KeyA', 'KeyS', 'KeyR'];
    if (gameKeys.includes(event.code)) {
        event.preventDefault();
    }
}

function handleWindowBlur() {
    // Stop emulation when window loses focus
    if (emulator && emulator.is_running && emulator.is_running()) {
        stopEmulation();
    }
}

// UI utilities
function updateStatus(message, type = 'info') {
    elements.statusText.textContent = message;
    elements.statusText.className = `status-${type}`;
}

function enableControls(enabled) {
    elements.startBtn.disabled = !enabled;
    elements.resetBtn.disabled = !enabled;
    elements.stepBtn.disabled = !enabled;
    elements.stopBtn.disabled = true; // Always start disabled
}

// Initialize when page loads
document.addEventListener('DOMContentLoaded', initialize);
