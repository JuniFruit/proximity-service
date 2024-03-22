<script lang="ts">
	import { Mobile, EnvelopeClosed, Gear } from 'svelte-radix';
	import { Button } from '@/components/ui/button/';
</script>

<div class="container_f">
	<div class="container_inner">
		<div class="button_container">
			<Button disabled variant="ghost">
				<EnvelopeClosed />
			</Button>
			<Button variant="outline" size="icon">
				<Mobile />
			</Button>
			<Button disabled variant="ghost">
				<Gear />
			</Button>
		</div>
	</div>
</div>

<style scoped>
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
