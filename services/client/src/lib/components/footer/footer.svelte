<script lang="ts">
	import uiStore from '@/stores/ui';
	import { Button } from '@/components/ui/button/';
	import { createEventDispatcher } from 'svelte';
	import { Actions } from '@/components/actions';
	import { ConfigDialog } from '../config-dialog';

	const dispatch = createEventDispatcher();

	function onSimMoveCalled() {
		dispatch('simulateMovement');
	}
</script>

<div class="container_f">
	<div class="container_inner">
		<div class="button_container">
			<Actions on:simulateMovement={onSimMoveCalled} />
			<Button class="main_button" disabled={$uiStore.isChoosingPoint} variant="outline" size="icon">
				{#if !$uiStore.isLoading}
					<img src="map.png" alt="map" class="h-8 w-8" />
				{:else}
					<div class="lds-ripple">
						<div></div>
						<div></div>
					</div>
				{/if}
			</Button>
			<ConfigDialog />
		</div>
	</div>
</div>

<style scoped lang="postcss">
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

	.lds-ripple {
		/* change color here */
		color: #1c4c5b;
	}
	.lds-ripple,
	.lds-ripple div {
		box-sizing: border-box;
	}
	.lds-ripple {
		display: inline-block;
		position: relative;
		width: 50px;
		height: 50px;
	}
	.lds-ripple div {
		position: absolute;
		border: 4px solid currentColor;
		opacity: 1;
		border-radius: 50%;
		animation: lds-ripple 1s cubic-bezier(0, 0.2, 0.8, 1) infinite;
	}
	.lds-ripple div:nth-child(2) {
		animation-delay: -0.5s;
	}
	@keyframes lds-ripple {
		0% {
			top: 16px;
			left: 16px;
			width: 8px;
			height: 8px;
			opacity: 0;
		}
		4.9% {
			top: 26px;
			left: 26px;
			width: 8px;
			height: 8px;
			opacity: 0;
		}
		5% {
			top: 26px;
			left: 26px;
			width: 8px;
			height: 8px;
			opacity: 1;
		}
		100% {
			top: 0;
			left: 0;
			width: 36px;
			height: 36px;
			opacity: 0;
		}
	}
</style>
