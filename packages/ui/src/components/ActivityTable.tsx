import type { ActivityLog } from '../data/mockData';

interface ActivityTableProps {
  logs: ActivityLog[];
  totalResults?: number;
  currentPage?: number;
  totalPages?: number;
}

const iconColorClasses: Record<string, string> = {
  blue: 'bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400',
  amber: 'bg-amber-100 dark:bg-amber-900/30 text-amber-600 dark:text-amber-400',
  purple: 'bg-purple-100 dark:bg-purple-900/30 text-purple-600 dark:text-purple-400',
  indigo: 'bg-indigo-100 dark:bg-indigo-900/30 text-indigo-600 dark:text-indigo-400',
  red: 'bg-red-100 dark:bg-red-900/30 text-red-600 dark:text-red-400',
  slate: 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400',
};

const statusClasses: Record<string, string> = {
  success: 'bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-400',
  conflict: 'bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-400',
  ignored: 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400',
};

const statusDotClasses: Record<string, string> = {
  success: 'bg-emerald-500',
  conflict: 'bg-yellow-500',
  ignored: 'bg-slate-500',
};

export function ActivityTable({
  logs,
  totalResults = 1284,
  currentPage = 1,
  totalPages = 129,
}: ActivityTableProps) {
  return (
    <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl overflow-hidden shadow-sm">
      <table className="w-full text-left border-collapse">
        <thead>
          <tr className="bg-slate-50 dark:bg-slate-800/50 border-b border-slate-200 dark:border-slate-800 text-[10px] uppercase tracking-[0.1em]">
            <th className="px-6 py-4 font-bold text-slate-500">Timestamp</th>
            <th className="px-6 py-4 font-bold text-slate-500">Filename</th>
            <th className="px-6 py-4 font-bold text-slate-500">Path</th>
            <th className="px-6 py-4 font-bold text-slate-500 text-right">Status</th>
          </tr>
        </thead>
        <tbody className="divide-y divide-slate-100 dark:divide-slate-800">
          {logs.map((log) => (
            <tr
              key={log.id}
              className="hover:bg-slate-50/50 dark:hover:bg-slate-800/30 transition-colors group"
            >
              <td className="px-6 py-4 text-sm text-slate-500 tabular-nums">
                {log.timestamp}
              </td>
              <td className="px-6 py-4">
                <div className="flex items-center gap-3">
                  <div
                    className={`w-8 h-8 rounded flex items-center justify-center ${
                      iconColorClasses[log.iconColor]
                    }`}
                  >
                    <span className="material-icons-round text-lg">{log.icon}</span>
                  </div>
                  <span className="text-sm font-medium">{log.filename}</span>
                </div>
              </td>
              <td className="px-6 py-4">
                <div className="flex items-center gap-2 text-xs text-slate-500">
                  <span className="bg-slate-100 dark:bg-slate-800 px-2 py-1 rounded">
                    {log.sourcePath}
                  </span>
                  <span className="material-icons-round text-sm">chevron_right</span>
                  <span className="bg-primary/10 text-primary px-2 py-1 rounded font-medium">
                    {log.destPath}
                  </span>
                </div>
              </td>
              <td className="px-6 py-4 text-right">
                <span
                  className={`inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-bold ${
                    statusClasses[log.status]
                  }`}
                >
                  <span
                    className={`w-1.5 h-1.5 rounded-full ${statusDotClasses[log.status]}`}
                  ></span>
                  {log.status.charAt(0).toUpperCase() + log.status.slice(1)}
                </span>
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      {/* Pagination */}
      <div className="px-6 py-4 bg-slate-50 dark:bg-slate-800/50 flex items-center justify-between border-t border-slate-200 dark:border-slate-800">
        <p className="text-xs text-slate-500">
          Showing 1 to {logs.length} of {totalResults.toLocaleString()} results
        </p>
        <div className="flex items-center gap-1">
          <button className="w-8 h-8 flex items-center justify-center rounded border border-slate-200 dark:border-slate-700 hover:bg-white dark:hover:bg-slate-700 text-slate-400">
            <span className="material-icons-round text-lg">chevron_left</span>
          </button>
          <button className="w-8 h-8 flex items-center justify-center rounded bg-primary text-white text-xs font-bold shadow-sm shadow-primary/30">
            {currentPage}
          </button>
          <button className="w-8 h-8 flex items-center justify-center rounded border border-slate-200 dark:border-slate-700 hover:bg-white dark:hover:bg-slate-700 text-xs font-medium text-slate-600 dark:text-slate-400">
            2
          </button>
          <button className="w-8 h-8 flex items-center justify-center rounded border border-slate-200 dark:border-slate-700 hover:bg-white dark:hover:bg-slate-700 text-xs font-medium text-slate-600 dark:text-slate-400">
            3
          </button>
          <span className="px-1 text-slate-400 text-xs">...</span>
          <button className="w-8 h-8 flex items-center justify-center rounded border border-slate-200 dark:border-slate-700 hover:bg-white dark:hover:bg-slate-700 text-xs font-medium text-slate-600 dark:text-slate-400">
            {totalPages}
          </button>
          <button className="w-8 h-8 flex items-center justify-center rounded border border-slate-200 dark:border-slate-700 hover:bg-white dark:hover:bg-slate-700 text-slate-400">
            <span className="material-icons-round text-lg">chevron_right</span>
          </button>
        </div>
      </div>
    </div>
  );
}
