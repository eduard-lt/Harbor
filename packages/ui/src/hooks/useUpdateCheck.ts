import { useState, useEffect, useCallback } from 'react';
import packageJson from '../../package.json';

const GITHUB_REPO = 'eduard-lt/Harbor';
const CHECK_UPDATES_KEY = 'harbor_check_updates';
const LAST_NOTIFIED_VERSION_KEY = 'harbor_last_notified_version';

interface UpdateState {
    available: boolean;
    version: string | null;
    url: string | null;
    loading: boolean;
    error: string | null;
}

export function useUpdateCheck() {
    const [updateState, setUpdateState] = useState<UpdateState>({
        available: false,
        version: null,
        url: null,
        loading: false,
        error: null,
    });

    // Initialize from localStorage, default to true
    const [checkUpdates, setCheckUpdates] = useState(() => {
        const stored = localStorage.getItem(CHECK_UPDATES_KEY);
        return stored === null ? true : stored === 'true';
    });

    const toggleCheckUpdates = useCallback(() => {
        setCheckUpdates((prev) => {
            const newValue = !prev;
            localStorage.setItem(CHECK_UPDATES_KEY, String(newValue));
            return newValue;
        });
    }, []);

    const checkForUpdates = useCallback(async () => {
        if (!checkUpdates) {
            setUpdateState((prev) => ({ ...prev, available: false, loading: false }));
            return;
        }

        setUpdateState((prev) => ({ ...prev, loading: true, error: null }));

        try {
            const response = await fetch(`https://api.github.com/repos/${GITHUB_REPO}/releases/latest`);

            if (!response.ok) {
                throw new Error(`Failed to fetch releases: ${response.statusText}`);
            }

            const data = await response.json();
            const latestTag = data.tag_name; // e.g., "v1.2.1"

            // Simple string comparison for now, assuming standard semantic versioning with 'v' prefix
            // If latestTag is lexically greater than currentVersion, and it's not the same
            // Note: This is a basic comparison. For robust semver, we might need a library, 
            // but for this project's scope, if we stick to vX.Y.Z, string compare works for simple increments 
            // as long as digits scale (e.g. 1.10 > 1.2). 
            // Better approach: strip 'v' and use localeCompare with numeric options or split.

            const cleanLatest = latestTag.replace(/^v/, '');
            const cleanCurrent = packageJson.version;

            const isNewer = compareVersions(cleanLatest, cleanCurrent) > 0;

            if (isNewer) {
                const lastNotified = localStorage.getItem(LAST_NOTIFIED_VERSION_KEY);
                // Only show if we haven't notified for this specific version yet OR if the user hasn't dismissed it
                // The requirement says: "another notification will be only if there will be a 1.2.2 release etc"
                // This implies if they dismissed 1.2.1, they don't see it again.

                const alreadyDismissed = lastNotified === latestTag;

                setUpdateState({
                    available: !alreadyDismissed,
                    version: latestTag,
                    url: data.html_url,
                    loading: false,
                    error: null,
                });
            } else {
                setUpdateState({
                    available: false,
                    version: latestTag,
                    url: data.html_url,
                    loading: false,
                    error: null,
                });
            }

        } catch (err) {
            console.error('Update check failed:', err);
            setUpdateState((prev) => ({
                ...prev,
                loading: false,
                error: err instanceof Error ? err.message : String(err)
            }));
        }
    }, [checkUpdates]);

    useEffect(() => {
        checkForUpdates();
    }, [checkForUpdates]);

    const dismissNotification = useCallback(() => {
        if (updateState.version) {
            localStorage.setItem(LAST_NOTIFIED_VERSION_KEY, updateState.version);
            setUpdateState((prev) => ({ ...prev, available: false }));
        }
    }, [updateState.version]);

    return {
        ...updateState,
        checkUpdates,
        toggleCheckUpdates,
        dismissNotification,
        refreshUpdateCheck: checkForUpdates
    };
}

/**
 * Returns:
 * 1 if v1 > v2
 * -1 if v1 < v2
 * 0 if v1 === v2
 */
function compareVersions(v1: string, v2: string): number {
    const p1 = v1.split('.').map(Number);
    const p2 = v2.split('.').map(Number);
    const len = Math.max(p1.length, p2.length);

    for (let i = 0; i < len; i++) {
        const n1 = p1[i] || 0;
        const n2 = p2[i] || 0;
        if (n1 > n2) return 1;
        if (n1 < n2) return -1;
    }
    return 0;
}
