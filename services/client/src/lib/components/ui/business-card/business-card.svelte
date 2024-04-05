<script lang="ts">
	import { Star } from 'svelte-radix';
	import { SH4, Small } from '@/components/ui/typography';
	import windowStore from '@/stores/window';
	import uiStore from '@/stores/ui';
	export let stars: number = 0;
	export let title: string;
	export let type: string;
	export let opensAt: number;
	export let closesAt: number;
	export let id: number;
	export let isSelected: boolean;
	let isMobile: boolean;

	function isOpenNow() {
		const hours = new Date().getHours();
		return hours >= opensAt && hours <= closesAt;
	}

	windowStore.subscribe((data) => {
		isMobile = data;
	});

	function onBusinessClick() {
		uiStore.update((data) => {
			data.businessSelected = id;
			return data;
		});
	}

	function stripTitle(str: string, max = 21) {
		if (!str) return '';

		if (str.length > max) {
			return str.slice(0, max) + '...';
		}
		return str;
	}
</script>

<div
	class={`card_container ${isSelected ? 'border border-sky-300' : ''}`}
	role="button"
	tabindex="0"
	on:keypress={() => {}}
	on:click={onBusinessClick}
>
	<div class="image_container">
		<img src="placeholder.png" alt="business" loading="lazy" />
	</div>
	<div class="body_container">
		<div class="flex flex-col gap-2">
			<SH4>{stripTitle(title, isMobile ? 21 : 40)}</SH4>
			<div class="stars_container">
				{#each Array(stars) as _}
					<Star class="h-3 w-3 text-emerald-500" />
				{/each}
				{#each Array(5 - stars) as _}
					<Star class="h-3 w-3 text-emerald-200" />
				{/each}
			</div>
			<div>
				<Small>{type}</Small>
				<Small
					>{isOpenNow() ? '' : 'closed. '}{`working hours: ${opensAt}:00 - ${closesAt}:00`}</Small
				>
			</div>
		</div>
	</div>
</div>

<style scoped lang="postcss">
	.card_container {
		@apply flex  w-full overflow-hidden rounded-xl bg-white;
	}
	.image_container {
		@apply h-28 w-24;
	}
	.image_container img {
		@apply h-auto w-full;
	}
	.body_container {
		@apply px-3 py-2;
	}
	.body_container > div {
		@apply flex flex-col gap-2;
	}

	.stars_container {
		@apply flex w-full;
	}
</style>
