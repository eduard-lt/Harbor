import { useState, useEffect, useCallback } from 'react';
import type { ServiceStatus } from '../lib/tauri';
import {
    getServiceStatus,
    startService,
    stopService,
    triggerOrganizeNow,
    getStartupEnabled,
    setStartupEnabled as setStartupEnabledApi,
    getDownloadDir,
    reloadConfig,
    resetToDefaults
} from '../lib/tauri';

export function useSettings() {
    const [serviceStatus, setServiceStatus] = useState<ServiceStatus>({ running: false });
    const [startupEnabled, setStartupEnabled] = useState(false);
    const [downloadDir, setDownloadDir] = useState('');
    const [loading, setLoading] = useState(true);
    const [organizing, setOrganizing] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const fetchStatus = useCallback(async () => {
        try {
            const [status, startup, dir] = await Promise.all([
                getServiceStatus(),
                getStartupEnabled(),
                getDownloadDir()
            ]);
            setServiceStatus(status);
            setStartupEnabled(startup);
            setDownloadDir(dir);
            setError(null);
        } catch (err) {
            console.error('Failed to fetch settings:', err);
            setError(err instanceof Error ? err.message : String(err));
        } finally {
            setLoading(false);
        }
    }, []);

    useEffect(() => {
        fetchStatus();
        // Poll service status every 5 seconds
        const interval = setInterval(() => {
            getServiceStatus().then(setServiceStatus).catch(console.error);
        }, 5000);
        return () => clearInterval(interval);
    }, [fetchStatus]);

    const toggleService = async () => {
        try {
            if (serviceStatus.running) {
                await stopService();
                setServiceStatus({ ...serviceStatus, running: false });
            } else {
                await startService();
                setServiceStatus({ ...serviceStatus, running: true });
            }
        } catch (err) {
            setError(err instanceof Error ? err.message : String(err));
            // Re-fetch to ensure sync
            fetchStatus();
        }
    };

    const toggleStartup = async () => {
        try {
            const newState = !startupEnabled;
            await setStartupEnabledApi(newState);
            setStartupEnabled(newState);
        } catch (err) {
            setError(err instanceof Error ? err.message : String(err));
            fetchStatus();
        }
    };

    const organizeNow = async () => {
        try {
            setOrganizing(true);
            const count = await triggerOrganizeNow();
            return count;
        } catch (err) {
            setError(err instanceof Error ? err.message : String(err));
            throw err;
        } finally {
            setOrganizing(false);
        }
    };

    const reload = async () => {
        try {
            await reloadConfig();
            await fetchStatus();
        } catch (err) {
            setError(err instanceof Error ? err.message : String(err));
            throw err;
        }
    }

    const reset = async () => {
        try {
            await resetToDefaults();
            await fetchStatus();
        } catch (err) {
            setError(err instanceof Error ? err.message : String(err));
            throw err;
        }
    }

    return {
        serviceStatus,
        startupEnabled,
        downloadDir,
        loading,
        organizing,
        error,
        toggleService,
        toggleStartup,
        organizeNow,
        reload,
        reset,
        refresh: fetchStatus,
    };
}
