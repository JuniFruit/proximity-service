<script lang="ts">
	import { BusinessCard } from '@/components/ui/business-card';
	import type { BusinessData } from '@/types/business.d';
	import * as Carousel from '@/components/ui/carousel';
	import windowStore from '@/stores/window';

	export let businesses: BusinessData[] = [];

	let isMobile: boolean;

	$: console.log(isMobile);

	windowStore.subscribe((data) => {
		isMobile = data;
	});
</script>

<Carousel.Root class="h-fit w-full">
	<Carousel.Content>
		{#each businesses as business}
			<Carousel.Item class={!isMobile ? 'basis-1/2' : ''}>
				<BusinessCard title={business.name} type={business.type} stars={business.stars} />
			</Carousel.Item>
		{/each}
	</Carousel.Content>
</Carousel.Root>
