import type { UIStore } from '@/types/stores';
import { writable } from 'svelte/store';

const initial: UIStore = {
	businessSelected: 0
};

const uiStore = writable(initial);

export default uiStore;
