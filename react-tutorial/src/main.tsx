import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
  // strict mode is a tool for highlighting potential problems in an application. It activates additional checks and warnings for its descendants. In development mode, it helps identify unsafe lifecycles, legacy API usage, and other issues that could lead to bugs or performance problems. Strict mode does not affect the production build of the application.
  // StrictMode is a wrapper component that you can use to wrap your entire application or specific parts of it. It does not render any visible UI, but it enables additional checks and warnings for its children. You can use StrictMode to help identify potential issues in your application and improve the overall quality of your code.
  // That means it gets rendered twice in development mode, but only once in production mode. This is intentional and is done to help identify potential issues in the application during development. The double rendering allows React to detect any side effects or issues that may arise from rendering components multiple times, which can help improve the overall quality of the code. However, in production mode, React optimizes the rendering process and only renders components once for better performance.
)
