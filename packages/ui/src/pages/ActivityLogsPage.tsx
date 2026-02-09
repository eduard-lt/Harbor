import { Header } from '../components/Header';
import { ActivityTable } from '../components/ActivityTable';
import { activityLogs } from '../data/mockData';

export function ActivityLogsPage() {
  return (
    <>
      <Header title="Activity Logs" subtitle="1,284 files anchored today" />
      <div className="flex-1 p-8 overflow-auto">
        <ActivityTable logs={activityLogs} />
      </div>
      <div className="h-1 bg-gradient-to-r from-primary/10 via-primary/60 to-primary/10 opacity-30"></div>
    </>
  );
}
