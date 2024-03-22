<svelte:head>
	<link
		rel="stylesheet"
		href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
		integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY="
		crossorigin=""
	/>
</svelte:head>
<div class="page_container">
  <div class="header_contianer">
    <Header />
  </div>
	<div id="map" bind:this={mapContainer}/>
	<div class="footer_container">
		<Footer />
	</div>
</div>

<script lang="ts">
	import { browser } from '$app/environment';
	import type { LatLngExpression, Map } from 'leaflet';
	import { onMount } from 'svelte';
	import { Footer } from '@/components/footer';
  import {Header} from '@/components/header';

	let L: any;
	let map: Map;
	let mapContainer: HTMLDivElement;
	const initialView: LatLngExpression = [39.8283, -98.5795];

	function init(container: HTMLElement) {
		map = createMap(container);
	}
	function createMap(container: HTMLElement) {
		let m = L.map(container, { preferCanvas: true }).setView(initialView, 5);
		L.tileLayer('https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png', {
			attribution: `&copy;<a href="https://www.openstreetmap.org/copyright" target="_blank">OpenStreetMap</a>,
	        &copy;<a href="https://carto.com/attributions" target="_blank">CARTO</a>`,
			subdomains: 'abcd',
			maxZoom: 14
		}).addTo(m);

		return m;
	}

	onMount(async () => {
		if (browser) {
			L = await import('leaflet');
			if (mapContainer) {
				init(mapContainer);
			}
		}
	});
</script>

<style scoped>
	#map {
		height: 100%;
		width: 100%;
		z-index: 10;
	}
	.page_container {
		@apply relative h-full w-full;
	}
	.footer_container {
		@apply absolute bottom-0 left-0 z-40 w-full;
	}
  .header_contianer {
    @apply absolute top-5 left-0 z-40 w-full h-fit;
  }
</style>
