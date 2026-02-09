import { useState } from 'react';
import { Header } from '../components/Header';
import { StatCard } from '../components/StatCard';
import { rules, recentActivity } from '../data/mockData';
import type { Rule } from '../data/mockData';

const iconColorClassesLight: Record<string, string> = {
  indigo: 'bg-indigo-100 text-indigo-600 dark:bg-indigo-900/20 dark:text-indigo-400',
  amber: 'bg-amber-100 text-amber-600 dark:bg-amber-900/20 dark:text-amber-400',
  slate: 'bg-slate-100 text-slate-500 dark:bg-slate-800 dark:text-slate-500',
};

export function RulesPage() {
  const [rulesState, setRulesState] = useState<Rule[]>(rules);

  const toggleRule = (id: string) => {
    setRulesState((prev) =>
      prev.map((rule) =>
        rule.id === id ? { ...rule, enabled: !rule.enabled } : rule
      )
    );
  };

  return (
    <>
      <Header
        title="Rules Management"
        subtitle="Define how Harbor handles your incoming files automatically."
      >
        <div className="relative group">
          <span className="material-icons-round absolute left-3 top-1/2 -translate-y-1/2 text-slate-500 text-lg">
            search
          </span>
          <input
            className="bg-slate-100 dark:bg-background-card border border-slate-200 dark:border-slate-700 text-slate-800 dark:text-slate-200 text-sm rounded-lg pl-10 pr-4 py-2 w-64 focus:ring-primary focus:border-primary transition-all outline-none"
            placeholder="Search rules..."
            type="text"
          />
        </div>
        <button className="bg-primary hover:bg-primary-dark text-white px-5 py-2.5 rounded-lg font-semibold flex items-center gap-2 transition-all shadow-lg shadow-primary/10">
          <span className="material-icons-round text-lg">add</span>
          New Rule
        </button>
      </Header>

      <div className="p-8 max-w-7xl mx-auto w-full overflow-auto">
        {/* Stats */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
          <StatCard
            icon="checklist"
            iconBgClass="bg-blue-100 dark:bg-blue-900/30"
            iconTextClass="text-blue-600 dark:text-blue-400"
            label="Active Rules"
            value={rulesState.filter((r) => r.enabled).length}
          />
          <StatCard
            icon="bolt"
            iconBgClass="bg-primary/10"
            iconTextClass="text-primary"
            label="Auto-Actions"
            value={1402}
          />
        </div>

        {/* Rules Table */}
        <div className="bg-white dark:bg-background-card rounded-xl border border-slate-200 dark:border-slate-800 overflow-hidden mb-12">
          <div className="overflow-x-auto">
            <table className="w-full text-left">
              <thead className="bg-slate-50 dark:bg-slate-800/50 text-slate-500 dark:text-slate-400 text-xs uppercase font-bold tracking-wider border-b border-slate-200 dark:border-slate-800">
                <tr>
                  <th className="px-6 py-4">Rule Name</th>
                  <th className="px-6 py-4">Extensions</th>
                  <th className="px-6 py-4">Destination</th>
                  <th className="px-6 py-4">Status</th>
                  <th className="px-6 py-4 text-right">Actions</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-100 dark:divide-slate-800 text-slate-800 dark:text-slate-200">
                {rulesState.map((rule) => (
                  <tr
                    key={rule.id}
                    className={`hover:bg-slate-50 dark:hover:bg-slate-800/30 transition-colors group ${
                      !rule.enabled ? 'opacity-50' : ''
                    }`}
                  >
                    <td className="px-6 py-5">
                      <div className="flex items-center gap-3">
                        <div
                          className={`w-10 h-10 rounded-lg flex items-center justify-center ${
                            iconColorClassesLight[rule.iconColor]
                          }`}
                        >
                          <span className="material-icons-round">{rule.icon}</span>
                        </div>
                        <span className="font-bold dark:text-white">{rule.name}</span>
                      </div>
                    </td>
                    <td className="px-6 py-5">
                      <div className="flex flex-wrap gap-1">
                        {rule.extensions.map((ext) => (
                          <span
                            key={ext}
                            className="px-2 py-0.5 bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 rounded font-mono text-xs border border-slate-200 dark:border-slate-700"
                          >
                            {ext}
                          </span>
                        ))}
                      </div>
                    </td>
                    <td className="px-6 py-5">
                      <span className="font-mono text-sm text-slate-500 dark:text-slate-400">
                        {rule.destination}
                      </span>
                    </td>
                    <td className="px-6 py-5">
                      <label className="relative inline-flex items-center cursor-pointer">
                        <input
                          type="checkbox"
                          className="sr-only peer"
                          checked={rule.enabled}
                          onChange={() => toggleRule(rule.id)}
                        />
                        <div className="w-9 h-5 bg-slate-200 dark:bg-slate-700 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-primary"></div>
                      </label>
                    </td>
                    <td className="px-6 py-5 text-right">
                      <div className="flex justify-end gap-2">
                        <button className="p-1.5 text-slate-400 dark:text-slate-500 hover:text-primary hover:bg-primary/10 rounded-md transition-colors">
                          <span className="material-icons-round text-xl">edit</span>
                        </button>
                        <button className="p-1.5 text-slate-400 dark:text-slate-500 hover:text-red-400 hover:bg-red-400/10 rounded-md transition-colors">
                          <span className="material-icons-round text-xl">delete_outline</span>
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        {/* Recent Activity */}
        <div className="bg-white dark:bg-background-card rounded-xl border border-slate-200 dark:border-slate-800 p-6">
          <div className="flex items-center justify-between mb-6">
            <h3 className="text-lg font-bold dark:text-white flex items-center gap-2">
              <span className="material-icons-round text-primary">history</span>
              Recent Activity
            </h3>
            <a
              className="text-primary text-sm font-semibold hover:text-primary-dark"
              href="#"
            >
              View All Logs
            </a>
          </div>
          <div className="overflow-hidden border border-slate-200 dark:border-slate-800 rounded-lg">
            <table className="w-full text-left">
              <thead className="bg-slate-50 dark:bg-slate-800/50 text-slate-500 dark:text-slate-400 text-xs uppercase font-bold tracking-wider">
                <tr>
                  <th className="px-6 py-4">File Name</th>
                  <th className="px-6 py-4">Status</th>
                  <th className="px-6 py-4">Action Taken</th>
                  <th className="px-6 py-4">Time</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-100 dark:divide-slate-800">
                {recentActivity.map((activity) => (
                  <tr
                    key={activity.id}
                    className="hover:bg-slate-50 dark:hover:bg-slate-800/30 transition-colors"
                  >
                    <td className="px-6 py-4">
                      <div className="flex items-center gap-3">
                        <span
                          className={`material-icons-round text-base ${
                            activity.iconColor === 'indigo'
                              ? 'text-indigo-400'
                              : activity.iconColor === 'amber'
                              ? 'text-amber-400'
                              : 'text-slate-500'
                          }`}
                        >
                          {activity.icon}
                        </span>
                        <span className="text-sm font-medium text-slate-800 dark:text-slate-200">
                          {activity.filename}
                        </span>
                      </div>
                    </td>
                    <td className="px-6 py-4">
                      <span
                        className={`px-2.5 py-1 text-[10px] font-bold rounded-full uppercase tracking-wide border ${
                          activity.status === 'success'
                            ? 'bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 border-green-200 dark:border-green-900/50'
                            : 'bg-slate-100 dark:bg-slate-800 text-slate-500 dark:text-slate-400 border-slate-200 dark:border-slate-700'
                        }`}
                      >
                        {activity.status}
                      </span>
                    </td>
                    <td className="px-6 py-4 text-sm text-slate-500 dark:text-slate-400">
                      {activity.actionTaken}
                    </td>
                    <td className="px-6 py-4 text-sm text-slate-400 dark:text-slate-500">
                      {activity.time}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>

      {/* Floating notification */}
      <div className="fixed bottom-6 right-6 flex items-center gap-3 bg-primary text-white px-5 py-3 rounded-lg shadow-2xl shadow-primary/20 z-50 border border-primary/30">
        <span className="material-icons-round text-lg">check_circle</span>
        <span className="text-sm font-medium tracking-tight">
          Rules are actively monitoring your folders.
        </span>
      </div>
    </>
  );
}
