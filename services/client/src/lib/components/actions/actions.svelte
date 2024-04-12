<script lang="ts">
	import * as Dialog from '@/components/ui/dialog';
	import { Small } from '../ui/typography';
	import { Button } from '../ui/button';
	import { createEventDispatcher } from 'svelte';
	import uiStore from '@/stores/ui';
	import { Layers } from 'svelte-radix';

	const dispatch = createEventDispatcher();

	let isDialogOpen = false;

	function simulateMovementStart() {
		if (!$uiStore.isSimMoving) {
			isDialogOpen = false;
		}
		dispatch('simulateMovement');
	}
</script>

<Dialog.Root onOpenChange={(val) => (isDialogOpen = val)} open={isDialogOpen}>
	<Dialog.Trigger>
		<Button disabled={$uiStore.isChoosingPoint} variant="ghost">
			<Layers />
		</Button>
	</Dialog.Trigger>
	<Dialog.Content class="md:max-w-[1025px]">
		<Dialog.Header>
			<Dialog.Title>Actions</Dialog.Title>
			<Dialog.Description>Simulate action</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<div class="flex items-center justify-between">
				<Small>Simulate movement</Small>
				<Button on:click={simulateMovementStart}
					>{$uiStore.isSimMoving ? 'Stop' : 'Choose point'}</Button
				>
			</div>
			<div class="grid grid-cols-4 items-center gap-4"></div>
		</div>
		<Dialog.Footer></Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
