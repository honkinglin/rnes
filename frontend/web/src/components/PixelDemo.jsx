import React from 'react'
import { motion } from 'framer-motion'

const PixelDemo = () => {
  return (
    <motion.div 
      className="pixel-demo"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.8, delay: 0.6 }}
    >
      <div className="pixel-demo-header">
        <h2>🎮 Pixel Game Style Demo</h2>
        <p>Experience the retro gaming aesthetic</p>
      </div>
      
      <div className="pixel-demo-grid">
        <motion.div 
          className="pixel-card"
          whileHover={{ scale: 1.05, rotate: 2 }}
          whileTap={{ scale: 0.95 }}
        >
          <div className="pixel-card-icon">🎯</div>
          <h3>Pixel Perfect</h3>
          <p>Crisp pixel borders and shadows</p>
        </motion.div>
        
        <motion.div 
          className="pixel-card"
          whileHover={{ scale: 1.05, rotate: -2 }}
          whileTap={{ scale: 0.95 }}
        >
          <div className="pixel-card-icon">🌈</div>
          <h3>Retro Colors</h3>
          <p>Classic gaming color palette</p>
        </motion.div>
        
        <motion.div 
          className="pixel-card"
          whileHover={{ scale: 1.05, rotate: 2 }}
          whileTap={{ scale: 0.95 }}
        >
          <div className="pixel-card-icon">⚡</div>
          <h3>Smooth Animations</h3>
          <p>Fluid motion and transitions</p>
        </motion.div>
        
        <motion.div 
          className="pixel-card"
          whileHover={{ scale: 1.05, rotate: -2 }}
          whileTap={{ scale: 0.95 }}
        >
          <div className="pixel-card-icon">📱</div>
          <h3>Responsive</h3>
          <p>Works on all devices</p>
        </motion.div>
      </div>
    </motion.div>
  )
}

export default PixelDemo
