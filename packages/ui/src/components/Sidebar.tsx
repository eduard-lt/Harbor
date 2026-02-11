import { NavLink } from 'react-router-dom';

interface NavItem {
  to: string;
  icon: string;
  label: string;
}

const navItems: NavItem[] = [
  { to: '/', icon: 'history', label: 'Activity Logs' },
  { to: '/rules', icon: 'rule', label: 'Rules' },
  { to: '/settings', icon: 'settings', label: 'Settings' },
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
      <nav className="flex-1 px-4 py-6 space-y-2">
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
    </aside>
  );
}
