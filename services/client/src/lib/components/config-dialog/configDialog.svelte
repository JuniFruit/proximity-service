<script lang="ts">
	import * as Dialog from '@/components/ui/dialog';
	import * as Command from '@/components/ui/command';
	import * as Popover from '@/components/ui/popover';
	import { Small } from '../ui/typography';
	import { Button } from '../ui/button';
	import uiStore from '@/stores/ui';
	import configStore from '@/stores/config';
	import messageStore from '@/stores/message';
	import { ChevronDown, Gear } from 'svelte-radix';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { Input } from '../ui/input';

	let isDialogOpen = false;
	let simModeSelector = false;
	let simModes = ['fly', 'drive'];

	function saveConfig() {
		try {
			const str = JSON.stringify($configStore);
			window.localStorage.setItem('proximity_config', str);
		} catch (error) {
			messageStore.update(() => 'Failed to save config. ' + error);
		}
		isDialogOpen = false;
	}

	function apply() {
		saveConfig();
		window.location.reload();
	}
	function onModeSelected(value: string) {
		$configStore.simMovementMode = value as 'drive' | 'fly';
	}

	onMount(() => {
		if (browser) {
			try {
				const config = JSON.parse(window.localStorage.getItem('proximity_config') || '');
				if (config) {
					configStore.update(() => ({ ...config }));
				}
			} catch (error) {
				console.warn('Config is not found');
			}
		}
	});
</script>

<Dialog.Root onOpenChange={(val) => (isDialogOpen = val)} open={isDialogOpen}>
	<Dialog.Trigger>
		<Button disabled={$uiStore.isChoosingPoint} variant="ghost">
			<Gear />
		</Button>
	</Dialog.Trigger>
	<Dialog.Content class="md:max-w-[1025px]">
		<Dialog.Header>
			<Dialog.Title>App configuration</Dialog.Title>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<div class="flex items-center justify-between">
				<Small>Max visible radius of businesses</Small>
				<Input
					type="number"
					min={0}
					max={20000}
					class="max-w-fit"
					bind:value={$configStore.maxVisibleRadius}
				></Input>
			</div>
			<div class="flex items-center justify-between">
				<Small>Radius of search</Small>
				<Input
					type="number"
					min={500}
					max={20000}
					class="max-w-fit"
					bind:value={$configStore.requestRadius}
				></Input>
			</div>
			<div class="flex items-center justify-between">
				<Small>Debounce radius</Small>
				<Input
					type="number"
					min={0}
					max={5000}
					class="max-w-fit"
					bind:value={$configStore.requestDebounceRadius}
				></Input>
			</div>
			<div class="flex items-center justify-between">
				<Small>Movement mode</Small>
				<Popover.Root bind:open={simModeSelector}>
					<Popover.Trigger asChild let:builder>
						<Button
							builders={[builder]}
							variant="outline"
							role="combobox"
							aria-expanded={simModeSelector}
							class="w-[200px] justify-between md:w-[300px]"
						>
							{$configStore.simMovementMode || 'fly'}
							<ChevronDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
						</Button>
					</Popover.Trigger>
					<Popover.Content class="w-[200px] p-0 md:w-[300px]">
						<Command.Root>
							<Command.Input placeholder="Choose mode" />
							<Command.Empty>No mode found.</Command.Empty>
							<Command.Group>
								{#each simModes as mode}
									<Command.Item value={mode} onSelect={onModeSelected}>
										{mode}
									</Command.Item>
								{/each}
							</Command.Group>
						</Command.Root>
					</Popover.Content>
				</Popover.Root>
			</div>
		</div>
		<Dialog.Footer>
			<Button type="submit" class="mt-2" on:click={saveConfig}>Save</Button>
			<Button type="submit" class="mt-2" on:click={apply}>Apply (Reload)</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
