<script lang="ts">
	import uiStore from '@/stores/ui';
	import { Button } from '@/components/ui/button/';
	import { createEventDispatcher } from 'svelte';
	import { Actions } from '@/components/actions';
	import { ConfigDialog } from '../config-dialog';

	const dispatch = createEventDispatcher();

	function onSimMoveCalled() {
		dispatch('simulateMovement');
	}
</script>

<div class="container_f">
	<div class="container_inner">
		<div class="button_container">
			<Actions on:simulateMovement={onSimMoveCalled} />
			<Button disabled={$uiStore.isChoosingPoint} variant="outline" size="icon">
				<img src="map.png" alt="map" class="h-8 w-8" />
			</Button>
			<ConfigDialog />
		</div>
	</div>
</div>

<style scoped lang="postcss">
	.container_f {
		@apply h-12 w-full bg-white;
		box-shadow: -0.25rem -0.15rem 0.5rem 0.25rem rgba(0, 0, 0, 0.1);
	}
	.container_inner {
		@apply flex h-full w-full items-center justify-center;
	}
	.button_container {
		@apply flex w-full items-center justify-evenly;
	}
</style>
