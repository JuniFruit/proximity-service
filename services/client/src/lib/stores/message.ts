import { writable } from 'svelte/store';

const messageStore = writable('');

export default messageStore;
