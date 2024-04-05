import { writable } from 'svelte/store';

const windowStore = writable(true); // is mobile

export default windowStore;
