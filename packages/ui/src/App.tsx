import { BrowserRouter, Routes, Route, useNavigate } from 'react-router-dom';
import { ThemeProvider } from './context/ThemeContext';
import { Layout } from './components/Layout';
import { ActivityLogsPage } from './pages/ActivityLogsPage';
import { RulesPage } from './pages/RulesPage';
import { SettingsPage } from './pages/SettingsPage';
import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';

function GlobalNavigationListener() {
  const navigate = useNavigate();

  useEffect(() => {
    const unlisten = listen<string>('navigate', (event) => {
      navigate(event.payload);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, [navigate]);

  return null;
}

function GlobalContextMenuListener() {
  useEffect(() => {
    const handleContextMenu = (e: MouseEvent) => {
      if (process.env.NODE_ENV === 'production') {
        e.preventDefault();
      }
    };

    document.addEventListener('contextmenu', handleContextMenu);
    return () => {
      document.removeEventListener('contextmenu', handleContextMenu);
    };
  }, []);

  return null;
}

import { useWindowSize } from './hooks/useWindowSize';

function App() {
  // Initialize window size persistence
  useWindowSize();

  return (
    <ThemeProvider>
      <BrowserRouter>
        <GlobalNavigationListener />
        <GlobalContextMenuListener />
        <Routes>
          <Route path="/" element={<Layout />}>
            <Route index element={<ActivityLogsPage />} />
            <Route path="rules" element={<RulesPage />} />
            <Route path="settings" element={<SettingsPage />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </ThemeProvider>
  );
}

export default App;
