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

function App() {
  return (
    <ThemeProvider>
      <BrowserRouter>
        <GlobalNavigationListener />
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
