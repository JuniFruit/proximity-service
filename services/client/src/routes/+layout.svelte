<script lang="ts">
	import * as Alert from '@/components/ui/alert';
	import '../app.pcss';
	import { ExclamationTriangle } from 'svelte-radix';
	import messageStore from '@/stores/message';
	import { onDestroy } from 'svelte';

	let message: string;
	export const ssr = false;

	const unsub = messageStore.subscribe((msg: string) => {
		message = msg;
	});

	function clearMessage() {
		messageStore.update(() => '');
	}

	onDestroy(unsub);
</script>

<main class="relative">
	<slot />
	{#if message}
		<div
			role="button"
			tabindex="0"
			class="absolute bottom-12 left-0 z-50 w-full"
			on:click={clearMessage}
			on:keydown={clearMessage}
		>
			<Alert.Root>
				<ExclamationTriangle class="h-4 w-4 text-yellow-300" />
				<Alert.Title>Heads up!</Alert.Title>
				<Alert.Description>{message}</Alert.Description>
			</Alert.Root>
		</div>
	{/if}
</main>
