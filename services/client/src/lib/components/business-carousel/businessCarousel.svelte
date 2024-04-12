<script lang="ts">
	import { BusinessCard } from '@/components/ui/business-card';
	import type { BusinessData } from '@/types/business.d';
	import * as Carousel from '@/components/ui/carousel';
	import windowStore from '@/stores/window';
	import uiStore from '@/stores/ui';

	export let businesses: BusinessData[] = [];
</script>

<Carousel.Root class="h-fit w-full">
	<Carousel.Content>
		{#each businesses as business (business.id)}
			<Carousel.Item class={!$windowStore ? 'basis-1/2' : 'basis-0'}>
				<BusinessCard
					isSelected={$uiStore.businessSelected === business.id}
					id={business.id}
					opensAt={business.opensAt}
					closesAt={business.closesAt}
					title={business.name}
					type={business.type}
					stars={business.stars}
				/>
			</Carousel.Item>
		{/each}
	</Carousel.Content>
</Carousel.Root>
