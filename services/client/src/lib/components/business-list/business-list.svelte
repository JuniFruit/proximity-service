<script lang="ts">
	import * as Dialog from '@/components/ui/dialog';
	import { createEventDispatcher } from 'svelte';
	import uiStore from '@/stores/ui';
	import BusinessCard from '../ui/business-card/business-card.svelte';
	import type { BusinessData } from '@/types/business';

	export let isDialogOpen = false;
	export let businesses: BusinessData[] = [];

	const dispatch = createEventDispatcher();

	function dialogStateChange(val: boolean) {
		if (val === false) {
			dispatch('dialogClosed');
		}
	}
</script>

<Dialog.Root onOpenChange={dialogStateChange} open={isDialogOpen}>
	<Dialog.Content class="h-full  md:max-w-full">
		<Dialog.Header>
			<Dialog.Title>Businesses</Dialog.Title>
			<Dialog.Description>Found near you</Dialog.Description>
		</Dialog.Header>
		<div class="overflow-y-scroll">
			<div class="grid gap-4 py-4">
				{#each businesses as business (business.id)}
					<BusinessCard
						on:itemClicked={() => dialogStateChange(false)}
						isSelected={$uiStore.businessSelected === business.id}
						id={business.id}
						opensAt={business.opensAt}
						closesAt={business.closesAt}
						title={business.name}
						type={business.type}
						stars={business.stars}
					/>
				{/each}
			</div>
		</div>

		<Dialog.Footer></Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
