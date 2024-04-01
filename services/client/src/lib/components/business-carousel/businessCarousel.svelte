<script lang="ts">
	import { BusinessCard } from '@/components/ui/business-card';
	import type { BusinessData } from '@/types/business.d';
	import * as Carousel from '@/components/ui/carousel';

	export let businesses: BusinessData[] = [];
</script>

<Carousel.Root class="h-fit w-full">
	<Carousel.Content>
		{#each businesses as business}
			<BusinessCard title={business.name} type={business.type} stars={business.stars} />
		{/each}
	</Carousel.Content>
	<!-- <Carousel.Previous /> -->
	<!-- <Carousel.Next /> -->
</Carousel.Root>
