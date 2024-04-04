<script lang="ts">
	import { Input } from '@/components/ui/input';
	import { Button } from '@/components/ui/button';
	import { Crosshair1 } from 'svelte-radix';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	function findMe() {
		dispatch('findMe');
	}
</script>

<div class="flex w-full justify-center">
	<div class="mx-auto flex items-center gap-5 space-x-2 px-2">
		<Input placeholder="Filter" class="bg-white" />
		<Button on:click={findMe} variant="outline" class="w-12" size="icon">
			<Crosshair1 />
		</Button>
	</div>
</div>
