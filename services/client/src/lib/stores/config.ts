import type { ConfigStore } from '@/types/stores';
import { writable } from 'svelte/store';

const initial: ConfigStore = {
	requestDebounceRadius: 100,
	requestRadius: 2000,
	maxVisibleRadius: 5000,
	simMovementMode: 'fly'
};

const configStore = writable(initial);

export default configStore;
