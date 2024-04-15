<script lang="ts">
	import { Button } from '@/components/ui/button';
	import { Crosshair1 } from 'svelte-radix';
	import { createEventDispatcher } from 'svelte';
	import { ChevronDown, Check } from 'svelte-radix';
	import * as Command from '@/components/ui/command';
	import * as Popover from '@/components/ui/popover';
	import { cn } from '@/utils';
	import { tick } from 'svelte';

	type Events = {
		findMe: null;
		locationSelected: [number, number];
	};

	type Location = {
		value: [number, number];
		label: string;
	};

	let locations: Location[] = [
		{
			value: [52.520007, 13.404954],
			label: 'Berlin'
		},
		{
			value: [51.507351, -0.127758],
			label: 'London'
		},
		{
			value: [55.755826, 37.6173],
			label: 'Moscow'
		},
		{
			value: [37.386052, -122.083851],
			label: 'Mointain View'
		},
		{
			value: [37.774929, -122.419416],
			label: 'San-Francisco'
		}
	];

	let open = false;
	let value = '';

	$: selectedValue = locations.find((f) => f.label === value)?.label ?? 'Select a location...';

	// We want to refocus the trigger button when the user selects
	// an item from the list so users can continue navigating the
	// rest of the form with the keyboard.
	function closeAndFocusTrigger(triggerId: string) {
		open = false;
		tick().then(() => {
			document.getElementById(triggerId)?.focus();
		});
	}

	const dispatch = createEventDispatcher<Events>();

	function findMe() {
		dispatch('findMe');
	}

	function onValueSelected(value: string) {
		dispatch('locationSelected', locations.find((item) => item.label === value)?.value || [0, 0]);
	}
</script>

<div class="flex w-full justify-center">
	<div class="mx-auto flex items-center gap-5 space-x-2 px-2">
		<Popover.Root bind:open let:ids>
			<Popover.Trigger asChild let:builder>
				<Button
					builders={[builder]}
					variant="outline"
					role="combobox"
					aria-expanded={open}
					class="w-[200px] justify-between md:w-[300px]"
				>
					{selectedValue}
					<ChevronDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
				</Button>
			</Popover.Trigger>
			<Popover.Content class="w-[200px] p-0 md:w-[300px]">
				<Command.Root>
					<Command.Input placeholder="Search location..." />
					<Command.Empty>No framework found.</Command.Empty>
					<Command.Group>
						{#each locations as location}
							<Command.Item
								value={location.label}
								onSelect={(currentValue) => {
									value = currentValue;
									onValueSelected(currentValue);
									closeAndFocusTrigger(ids.trigger);
								}}
							>
								<Check class={cn('mr-2 h-4 w-4', value !== location.label && 'text-transparent')} />
								{location.label}
							</Command.Item>
						{/each}
					</Command.Group>
				</Command.Root>
			</Popover.Content>
		</Popover.Root>
		<Button on:click={findMe} variant="outline" class="w-10" size="icon">
			<Crosshair1 />
		</Button>
	</div>
</div>
