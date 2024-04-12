import type { UIStore } from '@/types/stores';
import { writable } from 'svelte/store';

const initial: UIStore = {
	businessSelected: 0,
	isSimMoving: false,
	isChoosingPoint: false
};

const uiStore = writable(initial);

export default uiStore;
