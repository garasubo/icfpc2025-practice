import { useState } from 'react'
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="App">
      <header className="App-header">
        <h1>ICFPC 2025 Practice</h1>
        <div className="card">
          <button onClick={() => setCount((count) => count + 1)}>
            count is {count}
          </button>
          <p>
            React + TypeScript + Vite フロントエンド
          </p>
        </div>
        <p className="read-the-docs">
          API Server: Rust + axum + MySQL
        </p>
      </header>
    </div>
  )
}

export default App