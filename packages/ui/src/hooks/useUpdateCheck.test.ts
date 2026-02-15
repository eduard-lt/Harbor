import { renderHook, act, waitFor } from '@testing-library/react';
import { useUpdateCheck } from './useUpdateCheck';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

// Mock package.json
vi.mock('../../package.json', () => ({
    default: { version: '1.2.0' },
}));

describe('useUpdateCheck', () => {
    beforeEach(() => {
        localStorage.clear();
        vi.clearAllMocks();
        global.fetch = vi.fn();
    });

    afterEach(() => {
        vi.restoreAllMocks();
    });

    it('should initialize with defaults', () => {
        const { result } = renderHook(() => useUpdateCheck());
        expect(result.current.loading).toBe(true); // Starts loading immediately
        expect(result.current.available).toBe(false);
        expect(result.current.checkUpdates).toBe(true);
    });

    it('should detect an update', async () => {
        const mockRelease = {
            tag_name: 'v1.2.1',
            html_url: 'https://github.com/eduard-lt/Harbor-Download-Organizer/releases/tag/v1.2.1',
        };

        (global.fetch as any).mockResolvedValue({
            ok: true,
            json: async () => mockRelease,
        });

        const { result } = renderHook(() => useUpdateCheck());

        await waitFor(() => {
            expect(result.current.loading).toBe(false);
        });

        expect(result.current.available).toBe(true);
        expect(result.current.version).toBe('v1.2.1');
        expect(result.current.url).toBe(mockRelease.html_url);
    });

    it('should not detect update if version is same or older', async () => {
        const mockRelease = {
            tag_name: 'v1.2.0',
            html_url: 'https://github.com/eduard-lt/Harbor-Download-Organizer/releases/tag/v1.2.0',
        };

        (global.fetch as any).mockResolvedValue({
            ok: true,
            json: async () => mockRelease,
        });

        const { result } = renderHook(() => useUpdateCheck());

        await waitFor(() => {
            expect(result.current.loading).toBe(false);
        });

        expect(result.current.available).toBe(false);
    });

    it('should respect disabled updates', async () => {
        localStorage.setItem('harbor_check_updates', 'false');

        const { result } = renderHook(() => useUpdateCheck());

        expect(result.current.checkUpdates).toBe(false);
        expect(result.current.loading).toBe(false);
        expect(global.fetch).not.toHaveBeenCalled();
    });

    it('should toggle update checks', async () => {
        localStorage.setItem('harbor_check_updates', 'false');
        const { result } = renderHook(() => useUpdateCheck());

        expect(result.current.checkUpdates).toBe(false);

        act(() => {
            result.current.toggleCheckUpdates();
        });

        expect(result.current.checkUpdates).toBe(true);
        expect(localStorage.getItem('harbor_check_updates')).toBe('true');
    });

    it('should dismiss notification for a specific version', async () => {
        const mockRelease = {
            tag_name: 'v1.2.1',
            html_url: 'https://github.com/eduard-lt/Harbor-Download-Organizer/releases/tag/v1.2.1',
        };

        (global.fetch as any).mockResolvedValue({
            ok: true,
            json: async () => mockRelease,
        });

        const { result } = renderHook(() => useUpdateCheck());

        await waitFor(() => {
            expect(result.current.available).toBe(true);
        });

        act(() => {
            result.current.dismissNotification();
        });

        expect(result.current.available).toBe(false);
        expect(localStorage.getItem('harbor_last_notified_version')).toBe('v1.2.1');
    });

    it('should not show notification if already dismissed for this version', async () => {
        localStorage.setItem('harbor_last_notified_version', 'v1.2.1');

        const mockRelease = {
            tag_name: 'v1.2.1',
            html_url: 'https://github.com/eduard-lt/Harbor-Download-Organizer/releases/tag/v1.2.1',
        };

        (global.fetch as any).mockResolvedValue({
            ok: true,
            json: async () => mockRelease,
        });

        const { result } = renderHook(() => useUpdateCheck());

        await waitFor(() => {
            expect(result.current.loading).toBe(false);
        });

        expect(result.current.available).toBe(false);
    });
});
