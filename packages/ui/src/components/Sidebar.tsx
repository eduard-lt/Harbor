import { NavLink } from 'react-router-dom';
import { open } from '@tauri-apps/plugin-shell';

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

      {/* Footer Links */}
      <div className="p-4 border-t border-slate-200 dark:border-slate-800 space-y-2">
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
    </aside>
  );
}
