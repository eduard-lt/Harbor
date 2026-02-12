import { NavLink } from 'react-router-dom';
import { open } from '@tauri-apps/plugin-shell';
import { useSettings } from '../hooks/useSettings';

interface NavItem {
  to: string;
  icon: string;
  label: string;
}

const navItems: NavItem[] = [
  { to: '/', icon: 'rule', label: 'Rules' },
  { to: '/activity', icon: 'history', label: 'Activity Logs' },
  { to: '/settings', icon: 'settings', label: 'Settings' },
  { to: '/info', icon: 'info', label: 'Info & Guide' },
];

export function Sidebar() {
  const { serviceStatus, toggleService, loading } = useSettings();
  const serviceEnabled = serviceStatus.running;

  return (
    <aside className="w-20 lg:w-64 border-r border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 flex flex-col transition-all duration-300">
      {/* Logo */}
      <div className="p-6 flex items-center gap-3">
        <img src="/harbor.svg" alt="Harbor" className="w-10 h-10 object-contain" />
        <span className="text-xl font-bold tracking-tight hidden lg:block dark:text-white">Harbor</span>
      </div>

      {/* Navigation */}
      <nav className="flex-1 px-4 py-6 space-y-2 overflow-y-auto custom-scrollbar">
        {navItems.map((item) => (
          <NavLink
            key={item.to}
            to={item.to}
            className={({ isActive }) =>
              `flex items-center gap-4 px-4 py-3 rounded-lg transition-colors group ${isActive
                ? 'bg-primary/10 text-primary'
                : 'text-slate-500 dark:text-slate-400 hover:bg-slate-50 dark:hover:bg-slate-800'
              }`
            }
          >
            <span className="material-icons-round group-hover:text-primary">{item.icon}</span>
            <span className="font-medium hidden lg:block">{item.label}</span>
          </NavLink>
        ))}
      </nav>

      {/* Service Toggle & Footer Links */}
      <div className="p-4 border-t border-slate-200 dark:border-slate-800 space-y-4">
        {/* Service Toggle */}
        <div id="sidebar-service-toggle" className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-3 flex items-center justify-between group">
          <div className="flex items-center gap-2 overflow-hidden">
            <div className={`w-2 h-2 rounded-full flex-shrink-0 ${serviceEnabled ? 'bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.4)]' : 'bg-slate-400'}`}></div>
            <div className="flex flex-col min-w-0">
              <span className="text-xs font-bold text-slate-700 dark:text-slate-200 truncate">
                {serviceEnabled ? 'Active' : 'Stopped'}
              </span>
              <span className="text-[10px] text-slate-500 truncate hidden lg:block">
                {serviceEnabled ? 'Monitoring' : 'Paused'}
              </span>
            </div>
          </div>

          <label className="relative inline-flex items-center cursor-pointer flex-shrink-0">
            <input
              type="checkbox"
              className="sr-only peer"
              checked={serviceEnabled}
              onChange={toggleService}
              disabled={loading}
            />
            <div className="w-9 h-5 bg-slate-200 dark:bg-slate-700 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-slate-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-primary"></div>
          </label>
        </div>

        {/* External Links */}
        <div className="space-y-2">
          <button
            onClick={() => open('https://github.com/Eduard2609/Harbor')}
            className="w-full flex items-center gap-4 px-4 py-2 rounded-lg text-slate-500 dark:text-slate-400 hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors group"
            title="GitHub Repository"
          >
            <span className="material-icons-round text-xl group-hover:text-primary transition-colors">code</span>
            <span className="text-sm font-medium hidden lg:block group-hover:text-primary transition-colors">GitHub</span>
          </button>
          <button
            onClick={() => open('https://ko-fi.com/eduardolteanu')}
            className="w-full flex items-center gap-4 px-4 py-2 rounded-lg text-slate-500 dark:text-slate-400 hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors group"
            title="Buy me a coffee"
          >
            <span className="material-icons-round text-xl text-slate-400 group-hover:text-[#FF5E5B] transition-colors">favorite</span>
            <span className="text-sm font-medium hidden lg:block group-hover:text-[#FF5E5B] transition-colors">Donate</span>
          </button>
        </div>
      </div>
    </aside>
  );
}
