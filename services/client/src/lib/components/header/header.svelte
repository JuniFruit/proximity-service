<script lang="ts">
	import { Input } from '@/components/ui/input';
	import { Button } from '@/components/ui/button';
	import { Crosshair1 } from 'svelte-radix';
</script>

<div class="flex w-full justify-center">
	<div class="inner_container">
		<Input placeholder="Filter" class="bg-white" />
		<Button variant="outline" class="w-12" size="icon">
			<Crosshair1 />
		</Button>
	</div>
</div>

<style scoped lang="postcss">
	.inner_container {
		@apply mx-auto flex items-center gap-5 space-x-2 px-2;
	}
</style>
